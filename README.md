# LinuxTeasing (v1.0 Pixel Art Edition) 🐧

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-lightgrey)

> *"UTC is the only true time."*

**LinuxTeasing** is an ultra-lightweight, privacy-first terminal tool that politely "judges" you for abandoning UTC. If your system clock is set to a local timezone, you will be greeted by **Senior Tux**—a high-fidelity pixel art penguin who isn't angry, just disappointed.

---

## ✨ Features

- **Zero Latency**: Written in Rust, executes in **<10ms**. Invisible impact on shell startup.
- **Privacy First**: No network calls, no telemetry, no logging. All state is local.
- **Smart Judgment**:
  - **UTC**: Silent approval.
  - **Same Timezone**: Silent acceptance (judged once, never nagged again).
  - **New Timezone**: **JUDGMENT TRIGGERED**.
- **High-Fidelity Art**: Uses "Upper Half Block" (▀) rendering for crisp, truecolor pixel art.
- **Cross-Platform**: Works on Linux (bash/zsh/fish) and Windows (PowerShell).

---

## 🚀 Quick Start

### Linux / macOS
```bash
# Clone the repository
git clone https://github.com/kiet-ta/linux-teasing.git
cd linux-teasing

# Install
chmod +x install.sh
./install.sh
```

### Windows
```powershell
# Clone the repository
git clone https://github.com/kiet-ta/linux-teasing.git
cd linux-teasing

# Install
./install.ps1
```

For more detailed instructions, see the [Getting Started Guide](docs/getting-started.md).

---

## 🎨 The Experience

When you switch your system time from UTC to Local Time (e.g., `Asia/Ho_Chi_Minh`), you will see:

```text
        _
      _|_|_
     ( o o )
     (  -  )  <-- "Capiche?"
     //-=-\\
     (\_=_/)
      ^^ ^^

   "Back to Linux? Finally escaping the Windows dumpster."
```

*(Note: The ASCII above is a simplified representation. The actual tool renders high-fidelity pixel art.)*

---

## 🛠️ Configuration

State is stored locally in standard configuration paths:

- **Linux**: `~/.config/linux-teasing/state.json`
- **Windows**: `%APPDATA%/LinuxTeasing/config/state.json`

To reset the judgment (and see the penguin again), simply delete this file.

---

## 🤝 Contributing

Contributions are welcome! Please follow standard Rust conventions.

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.
