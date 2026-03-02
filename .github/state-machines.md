# LinuxTeasing — State Machine Reference

> Source of truth: read the `.rs` files directly. This document is generated from code analysis.  
> All diagrams use Mermaid.js v11 `stateDiagram-v2` and `flowchart` syntax.

---

## Table of Contents

1. [Binary Lifecycle](#1-binary-lifecycle)
2. [Timezone Judgment](#2-timezone-judgment)
3. [State File Persistence](#3-state-file-persistence)
4. [Pixel Renderer](#4-pixel-renderer)
5. [Install Script](#5-install-script)
6. [Cross-component Data Flow](#6-cross-component-data-flow)

---

## 1. Binary Lifecycle

**File:** `src/main.rs`

The top-level orchestrator. The binary runs once, takes exactly one path, then exits with code `0`.

```mermaid
stateDiagram-v2
    direction LR

    [*] --> Judging : binary starts

    Judging --> Silent   : Judgment::None
    Judging --> Guilty   : Judgment::Guilty

    Silent --> SaveState : update_state_after_judgment()
    SaveState --> [*]    : exit(0)

    Guilty --> RenderArt  : renderer::render()
    RenderArt --> PrintMsg : message::get_message()
    PrintMsg --> SaveState2 : update_state_after_judgment()
    SaveState2 --> [*]    : exit(0)
```

**Key invariant:** `update_state_after_judgment()` is called on **both** branches. State is always synced regardless of outcome.

---

## 2. Timezone Judgment

**File:** `src/detector.rs`

The decision engine. Produces a `Judgment` enum consumed by `main.rs`. No side effects — state is **not** written here, only read.

```mermaid
stateDiagram-v2
    direction TB

    [*] --> CheckUTC : Local::now().offset()

    CheckUTC --> EmitNone   : local_minus_utc == 0\n(UTC timezone)
    CheckUTC --> LoadState  : offset != 0

    LoadState --> CompareTimezone : AppState::load()

    CompareTimezone --> EmitNone   : current_tz == last_known_tz\n(same as recorded)
    CompareTimezone --> EmitGuilty : current_tz != last_known_tz\n(new non-UTC timezone)

    EmitNone   --> [*] : Judgment::None
    EmitGuilty --> [*] : Judgment::Guilty
```

**Offset string format:** `chrono::FixedOffset::to_string()` produces `+07:00`, not an IANA name.  
Two IANA zones at the same offset (e.g. `Asia/Bangkok` and `Asia/Ho_Chi_Minh`, both `+07:00`) are treated as **identical** — intentional design choice.

```mermaid
flowchart TD
    A["Local::now().offset()"] --> B{local_minus_utc == 0?}
    B -->|Yes — UTC| C["return Judgment::None"]
    B -->|No — non-UTC| D["current_tz = offset.to_string()"]
    D --> E["AppState::load()"]
    E --> F{current_tz == state.last_known_timezone?}
    F -->|Yes — same tz| G["return Judgment::None"]
    F -->|No — tz changed| H["return Judgment::Guilty"]
```

---

## 3. State File Persistence

**File:** `src/state.rs`

Manages `state.json` via `AppState`. Two operations: `load()` (read-or-default) and `save()` (write-or-create).

```mermaid
stateDiagram-v2
    direction LR

    %% --- LOAD path ---
    [*]          --> CheckFile    : AppState::load()
    CheckFile    --> ReadContent  : file exists
    CheckFile    --> ReturnDefault : file absent

    ReadContent  --> ParseJSON    : fs::read_to_string OK
    ReadContent  --> ReturnDefault : fs::read_to_string ERR

    ParseJSON    --> ReturnLoaded  : serde_json::from_str OK
    ParseJSON    --> ReturnDefault : serde_json::from_str ERR

    ReturnLoaded  --> [*] : AppState { last_known_timezone, last_judgment_timestamp }
    ReturnDefault --> [*] : AppState::default() — empty strings / 0

    %% --- SAVE path ---
    [*]           --> EnsureDir   : AppState::save()
    EnsureDir     --> WriteFile   : fs::create_dir_all OK
    EnsureDir     --> ErrReturn   : fs::create_dir_all ERR

    WriteFile     --> [*]         : Ok(()) — file written
    WriteFile     --> ErrReturn   : fs::write ERR
    ErrReturn     --> [*]         : Err(io::Error) — silently ignored in main
```

**State file paths:**

| Platform | Path |
|----------|------|
| Linux    | `~/.config/linux-teasing/state.json` |
| Windows  | `%APPDATA%\LinuxTeasing\config\state.json` |
| Fallback | `.config/linux-teasing/state.json` (relative) |

**Schema:**
```json
{
  "last_known_timezone": "+07:00",
  "last_judgment_timestamp": 1740873600
}
```

**Reset trigger:** Delete the state file → next run with any non-UTC timezone will fire `Judgment::Guilty`.

---

## 4. Pixel Renderer

**File:** `src/renderer.rs`

Renders `SENIOR_TUX` (`&[&str]` color-key grid) to truecolor terminal output.  
Consumes rows **two at a time**, pairing `top_row[i]` + `bottom_row[i+1]` into a single terminal line using `▀` (U+2580 Upper Half Block).

### 4a. Row Iteration State Machine

```mermaid
stateDiagram-v2
    direction TB

    [*]         --> RowLoop     : render() called
    RowLoop     --> PairRows    : i += 2 (step_by(2))
    PairRows    --> ColLoop     : top_row = SENIOR_TUX[i]\nbottom_row = SENIOR_TUX[i+1] or ""
    ColLoop     --> PixelState  : for col in 0..max_len
    PixelState  --> ColLoop     : next column
    ColLoop     --> RowLoop     : row pair exhausted → print \n
    RowLoop     --> Flush       : all rows exhausted
    Flush       --> [*]         : stdout.flush() → Ok(())
```

### 4b. Per-Pixel Color State Machine

Each character position resolves `top_color` and `bottom_color` via `get_color()`, then enters one of four states:

```mermaid
stateDiagram-v2
    direction LR

    [*] --> ResolveColors : get_color(top_char), get_color(bottom_char)

    ResolveColors --> BothTransparent : top=None, bottom=None
    ResolveColors --> TopOnly         : top=Some(c), bottom=None
    ResolveColors --> BottomOnly      : top=None, bottom=Some(c)
    ResolveColors --> BothColored     : top=Some(c1), bottom=Some(c2)

    BothTransparent --> [*] : Print(" ")

    TopOnly --> [*]     : SetFG(c)\nSetBG(Reset)\nPrint(▀)\nReset

    BottomOnly --> [*]  : SetFG(Reset)\nSetBG(c)\nPrint(▀)\nReset

    BothColored --> [*] : SetFG(c1)\nSetBG(c2)\nPrint(▀)\nReset
```

### 4c. Color Key → RGB Mapping

| Char | State Name | RGB | Visual role |
|------|-----------|-----|-------------|
| `B` | Black body | `#222222` | Body outline |
| `W` | White belly | `#F0F0F0` | Chest / belly |
| `Y` | Yellow | `#FFAE00` | Beak, feet |
| `G` | Gray mug | `#5D5D5D` | Coffee mug frame |
| `C` | Coffee brown | `#6F4E37` | Coffee liquid |
| `Z` | Light gray | `#AAAAAA` | Eyes |
| ` ` | Transparent | _(terminal BG)_ | Empty space |

---

## 5. Install Script

**File:** `install.sh` (Linux/macOS) — `install.ps1` (Windows)

```mermaid
stateDiagram-v2
    direction TB

    [*]              --> BuildRelease   : ./install.sh

    BuildRelease     --> CheckWritable  : cargo build --release OK
    BuildRelease     --> [*]            : cargo ERR → set -e aborts

    CheckWritable    --> DirectCopy     : /usr/local/bin is writable
    CheckWritable    --> SudoCopy       : not writable

    DirectCopy       --> DetectShell    : cp binary OK
    SudoCopy         --> DetectShell    : sudo cp binary OK

    DetectShell      --> AppendBash     : $SHELL ends with /bash
    DetectShell      --> AppendZsh      : $SHELL ends with /zsh
    DetectShell      --> AppendFish     : $SHELL ends with /fish
    DetectShell      --> WarnUnknown    : unrecognised shell

    AppendBash       --> CheckDuplicate : target = ~/.bashrc
    AppendZsh        --> CheckDuplicate : target = ~/.zshrc
    AppendFish       --> CheckDuplicate : target = ~/.config/fish/config.fish

    CheckDuplicate   --> SkipAppend     : grep finds "linux-teasing" already present
    CheckDuplicate   --> WriteStartup   : not found

    WriteStartup     --> [*]            : echo "linux-teasing" >> config ✓
    SkipAppend       --> [*]            : already installed — skip
    WarnUnknown      --> [*]            : print manual instruction, exit 0
```

---

## 6. Cross-component Data Flow

End-to-end sequence from shell open to screen output.

```mermaid
sequenceDiagram
    autonumber
    participant Shell
    participant main
    participant detector
    participant state as state.rs (AppState)
    participant renderer
    participant message

    Shell->>main: spawn linux-teasing binary

    main->>detector: judge()
    detector->>detector: Local::now().offset() — check UTC
    alt UTC timezone
        detector-->>main: Judgment::None
    else non-UTC
        detector->>state: AppState::load()
        state-->>detector: last_known_timezone (or default "")
        alt same as last known
            detector-->>main: Judgment::None
        else timezone changed
            detector-->>main: Judgment::Guilty
        end
    end

    alt Judgment::None
        main->>state: update_state_after_judgment() → save current tz
        main->>Shell: exit(0) — silent
    else Judgment::Guilty
        main->>renderer: render()
        renderer->>Shell: crossterm truecolor ▀ blocks (SENIOR_TUX art)
        main->>message: get_message()
        message-->>main: random &'static str (gold #FFAE00 styled)
        main->>Shell: println! quote
        main->>state: update_state_after_judgment() → save current tz
        main->>Shell: exit(0)
    end
```

---

## Appendix: State Transition Summary Table

| Machine | States | Trigger | Terminal state |
|---------|--------|---------|---------------|
| Binary Lifecycle | Judging → Silent\|Guilty → SaveState | binary spawn | exit(0) always |
| Timezone Judgment | CheckUTC → LoadState → Compare | `judge()` call | `Judgment::None` or `::Guilty` |
| State File | CheckFile → Read → Parse → Return | `load()` / `save()` | `AppState` struct or `Err` |
| Pixel Renderer | RowLoop → ColLoop → PixelState (×4) | `render()` call | `Ok(())` or `io::Err` |
| Install Script | Build → Copy → DetectShell → Append | `./install.sh` | config written or warned |
