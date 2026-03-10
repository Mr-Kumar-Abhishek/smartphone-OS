# Redme-9A OS – Microkernel Operating System for Redmi 9A Smartphone

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://rust-lang.org)
[![Platform: Redmi 9A](https://img.shields.io/badge/Platform-Redmi%209A-green.svg)](https://www.mi.com/global/product/redmi-9a/)

A custom, safety‑first microkernel operating system written in Rust, targeting the Redmi 9A smartphone. This project aims to deliver a lightweight, secure, and reliable OS that leverages Rust’s memory‑safety guarantees while providing a modern microkernel architecture.

## 🚀 Key Features

- **Microkernel Design** – Core kernel provides only essential services (IPC, scheduling, memory management); all other functionality runs in user‑space servers.
- **Written in Rust** – Zero‑cost abstractions, no undefined behavior, and guaranteed memory‑safety without a garbage collector.
- **Smartphone‑Specific Optimizations** – Tailored for the Redmi 9A’s MediaTek Helio G25 SoC, with drivers for display, touch, storage, and cellular modem.
- **Real‑Time Capable** – Predictable scheduling and low‑latency IPC suitable for real‑time tasks.
- **Modular & Extensible** – Add or replace system components (file systems, network stacks, GUI) without kernel modification.
- **Comprehensive Documentation** – Includes a full Software Requirements Specification (SRS) and Software Design Document (SDD).

## 📱 Hardware Target

| Component           | Specification                                                                 |
|---------------------|-------------------------------------------------------------------------------|
| **Device**          | Redmi 9A (Model: M2006C3LG)                                                   |
| **SoC**             | MediaTek Helio G25 (12 nm, 8× Cortex‑A53 up to 2.0 GHz)                       |
| **GPU**             | PowerVR GE8320 @ 650 MHz                                                      |
| **RAM**             | 2 GB LPDDR4X                                                                  |
| **Internal Storage**| 32 GB eMMC 5.1                                                                |
| **Display**         | 6.53″ IPS LCD, 720×1600 (HD+)                                                 |
| **Battery**         | 5000 mAh                                                                      |
| **Connectivity**    | 4G LTE, Wi‑Fi 802.11 b/g/n, Bluetooth 5.0, GPS, GLONASS                       |

The kernel is being developed to run **bare‑metal** on this hardware, booting via a custom bootloader or U‑Boot.

## 📚 Documentation

- **[Software Requirements Specification (SRS)](docs/SRS.md)** – Detailed functional and non‑functional requirements.
- **[Software Design Document (SDD)](docs/SDD.md)** – Architectural overview, component design, and implementation plans.
- **[Cargo.toml](Cargo.toml)** – Project metadata, dependencies, and build profiles.
- **[Source Code](src/)** – The kernel and system‑server source tree.

## 🛠 Getting Started

### Prerequisites

- **Rust toolchain** (nightly, with `rust-src` component) – install via [rustup](https://rustup.rs):
  ```bash
  rustup toolchain install nightly
  rustup component add rust-src --toolchain nightly
  ```
- **QEMU** (for emulation) – `qemu-system-arm` or `qemu-system-aarch64`.
- **Cross‑compilation tools** – `aarch64‑none‑elf‑gcc` or `arm‑none‑eabi‑gcc`.
- **Cargo‑make** (optional) – `cargo install cargo‑make` for simplified build workflows.

### Building the Kernel

1. Clone the repository:
   ```bash
   git clone https://github.com/Mr-Kumar-Abhishek/smartphone-OS.git
   cd smartphone-OS
   ```

2. Build the kernel for the target (AArch64):
   ```bash
   cargo build --target=aarch64‑unknown‑none‑softfloat --profile=kernel
   ```
   *Note: Custom target specifications and build profiles are defined in `.cargo/config.toml`.*

3. Generate the bootable image (requires a bootloader):
   ```bash
   cargo make image
   ```

### Running in QEMU

A QEMU launch script is provided to test the kernel in a virtualized environment:

```bash
./scripts/run-qemu.sh
```

This will start a QEMU instance with the kernel loaded, simulating the Redmi 9A’s core hardware.

### Flashing to Device (Experimental)

> **Warning:** Flashing a custom kernel can brick your device. Proceed only if you understand the risks and have a recovery method.

1. Unlock the bootloader (requires Mi Unlock tool).
2. Enter fastboot mode (`adb reboot bootloader`).
3. Flash the generated image:
   ```bash
   fastboot flash boot target/aarch64‑unknown‑none‑softfloat/kernel/boot.img
   ```
4. Reboot: `fastboot reboot`.

## 📁 Project Structure

```
redme-9a-os/
├── Cargo.toml              # Rust project manifest
├── .cargo/
│   └── config.toml         # Cross‑compilation and profile settings
├── src/
│   ├── main.rs             # Kernel entry point and early initialization
│   ├── arch/               # Architecture‑specific code (AArch64)
│   ├── kernel/             # Microkernel core (scheduler, IPC, memory)
│   ├── drivers/            # Hardware drivers (display, touch, storage, etc.)
│   ├── services/           # User‑space system servers
│   └── libs/               # Shared utilities (logging, collections, etc.)
├── docs/
│   ├── SRS.md              # Software Requirements Specification
│   └── SDD.md              # Software Design Document
├── scripts/                # Build, run, and flashing scripts
├── tests/                  # Unit and integration tests
└── target/                 # Build output (ignored by Git)
```

## 📈 Development Status

| Component               | Status                     | Notes                                      |
|-------------------------|----------------------------|--------------------------------------------|
| **Bootloader**          | 🔄 In Progress             | U‑Boot port & custom second‑stage loader   |
| **Microkernel Core**    | 🟡 Partial                 | Scheduler, IPC, memory manager underway    |
| **Hardware Drivers**    | 🟡 Partial                 | UART, GPIO, timer working; display WIP     |
| **System Services**     | ⚪ Not Started             | File‑system, network, GUI servers planned  |
| **User‑land API**       | ⚪ Not Started             | System‑call interface and libc stub        |
| **Testing**             | 🟡 Early                   | QEMU‑based unit tests; no on‑device tests  |

**Legend:** ⚪ Not Started · 🟡 Partial · 🔄 In Progress · ✅ Complete

## 👥 Contributing

We welcome contributions from the community! If you’re interested in helping build a Rust‑based OS for smartphones, please read the following guidelines.

### How to Contribute

1. **Fork the repository** and create a feature branch.
2. **Discuss major changes** by opening an issue first.
3. **Follow the coding standards** (see below).
4. **Write tests** for new functionality.
5. **Submit a pull request** with a clear description of the changes.

### Coding Standards

- **Rustfmt:** All code must be formatted with `rustfmt` (using the nightly toolchain).
- **Clippy:** Run `cargo clippy -- -D warnings` and fix any warnings before submitting.
- **Documentation:** Public APIs must be documented with `///` doc‑comments.
- **Commits:** Use [Conventional Commits](https://www.conventionalcommits.org/) style.
- **Safety:** Unsafe code must be justified with a `// SAFETY:` comment explaining the invariants.

### Issue Labels

- `good‑first‑issue` – Suitable for newcomers.
- `help‑wanted` – Extra hands needed.
- `bug` – Something isn’t working.
- `enhancement` – Improvement or new feature.

## 📄 License

This project is dual‑licensed under either:

- **[MIT License](LICENSE-MIT)**
- **[Apache License, Version 2.0](LICENSE-APACHE)**

at your option. See the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for details.

## 🙏 Acknowledgments

- The **Rust language team** for creating a systems language that makes safe systems programming possible.
- The **embedded‑rust community** for invaluable resources and crates (`cortex‑m`, `bare‑metal`, etc.).
- **QEMU developers** for providing a powerful emulation platform.
- **Xiaomi** for producing the Redmi 9A, an affordable and widely available hardware target.
- All **contributors and supporters** who help move this project forward.

---

*This README is a living document – please help keep it up‑to‑date by submitting improvements via pull requests.*