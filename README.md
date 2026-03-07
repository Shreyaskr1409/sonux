# Listen Listen

A modular music player composed of a background daemon and multiple frontends.

The architecture separates playback logic from user interfaces:

* **Daemon** — written in **C/C++**, communicates with **GStreamer** and manages playback.
* **GUI** — written in **Rust**.
* **TUI** — planned but not implemented yet.

The goal is to keep the playback engine independent so that multiple clients (GUI, TUI, scripts) can interact with the same daemon.

---

# Architecture

```
listen-listen
│
├── src/
│   ├── daemon/     # Playback daemon (C)
│   ├── common/     # Shared logic used by daemon/tests (C/C++)
│   ├── gui/        # Rust GUI client
│   └── tui/        # Future TUI client
│
├── build/          # Generated build output
└── Makefile
```

### Components

**Daemon**

* Handles playback
* Communicates with GStreamer
* Will manage queue, playlists, and library scanning

**GUI**

* Rust-based graphical interface
* Communicates with the daemon

**TUI (planned)**

* Terminal interface
* Not implemented yet

---

# Dependencies

## System dependencies

Required for building the daemon:

* gcc
* g++
* pkg-config
* gstreamer-1.0
* taglib

Example (Arch Linux):

```bash
sudo pacman -S gcc pkgconf gstreamer taglib
```

Example (Ubuntu/Debian):

```bash
sudo apt install build-essential pkg-config libgstreamer1.0-dev libtag1-dev
```

---

## Rust dependencies

The GUI is built using Rust, so you also need:

* rust
* cargo

Install with:

```bash
curl https://sh.rustup.rs -sSf | sh
```

---

# Installation

Clone the repository:

```bash
git clone --recursive https://github.com/Shreyaskr1409/listen-listen
cd listen-listen
```

---

# Build

The project uses a simple **Makefile** to orchestrate builds.

## Build everything (release GUI + daemon)

```bash
make
```

This builds:

```
build/listen-gui
build/listen-daemon
```

---

## Debug build

```bash
make debug
```

Builds:

```
build/listen-gui
build/listen-daemon
build/test-common
```

`test-common` is useful for testing code in `src/common`.

---

## Build individual components

### GUI (debug)

```bash
make debug-gui
```

### Daemon

```bash
make debug-daemon
```

---

# Output binaries

All compiled binaries are placed in:

```
build/
```

Example:

```
build/
├── listen-daemon
├── listen-gui
└── test-common
```

Object files are stored under:

```
build/obj/
```

---

# Cleaning the build

```bash
make clean
```

Removes the entire `build/` directory.

---

# Current Status

The project is still in an early stage.

Working:

* Basic daemon build
* GStreamer integration
* Rust GUI build pipeline

Planned next steps:

* Folder scanning
* Queue system
* Playlist management
* Gapless playback
* HTTP/stream URI support
* TUI client
* Improved daemon-client protocol

---
