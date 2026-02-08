# Security Policy

Thank you for your interest in the security of **LinuxTeasing**. As a tool that runs on user terminals (often with elevated privileges during installation), we take security seriously.

## Supported Versions

We only support security updates for the latest release. Please ensure you are running the most recent version before reporting a vulnerability.

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| < 1.0.0 | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability within this project, please prioritize **Responsible Disclosure**.

**DO NOT** open a public GitHub Issue for security vulnerabilities. This allows malicious actors to exploit the vulnerability before we can fix it.

### How to Report
Please email the maintainers directly at:
📧 **trananhkiet21082005@gmail.com**

Please include:
1.  **Type of issue** (e.g., Buffer Overflow, Path Injection, Dependency Vulnerability).
2.  **Full steps to reproduce** the issue.
3.  **Proof of Concept (PoC)** code or screenshots (if applicable).

### Our Response Process
1.  **Acknowledgment:** We will acknowledge your report within **48 hours**.
2.  **Assessment:** We will validate the vulnerability and determine its severity.
3.  **Fix:** We will prepare a patch and release a new version.
4.  **Disclosure:** Once the fix is released, we will publicly credit you (if desired) and publish a security advisory.

## Scope

### In Scope
- The `linux-teasing` binary logic.
- The installation scripts (`install.sh`, `install.ps1`).
- Configuration file handling (`state.json`).

### Out of Scope
- Vulnerabilities in the user's Operating System or Terminal Emulator.
- Issues related to user-customized build environments (e.g., modified `Cargo.toml`).

## Supply Chain Security (For Users)

Since this is a CLI tool distributed via source/binary:
- We recommend installing via the provided release binaries or building from source using `cargo build --release`.
- Always verify the source code if you are running in a sensitive environment.

---
*This policy is subject to change.*
