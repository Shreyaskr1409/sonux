# Listen Listen
A TUI and GUI music player built with C and Go.

This music player consists of:
- a daemon in C which runs a server to interact with GStreamer
- a GUI
- a TUI (going to be) built with Go using [@charmbracelet's](https://github.com/charmbracelet) [bubbletea](https://github.com/charmbracelet/bubbletea) and [lipgloss](https://github.com/charmbracelet/lipgloss).

## Installation

1. Install using git:
```bash
git clone --recursive https://github.com/Shreyaskr1409/listen-listen listen-listen
cd listen-listen/
```

2. Build the server
```bash
make build-server
```

3. Build the GUI (requires Clay and Raylib)

```bash
# make build-gui (yet to be developed)
```

4. Build the TUI (development yet to start)
```bash
# make build-tui (yet to be developed)
```

## Current Status
The daemon setup is done enough to do basic testings with GUI.
Implementation for gapless playback is yet to be done, but I will be implementing it very soon.
This project is at a very early stage with only a basic daemon working.

For now the current plan is to implement the following:
- Common setup to
  1. Scan a user given folder
  2. Play the songs in a queue
  3. Add playlist management
- Add gapless playback to the daemon.
- Add scripts to test the files easily.
- Handle cases where the URI is not a file but is available through something like HTTP.

While dealing with different OS, I prefer to have different Makefile for different platforms instead of having
a CMake or Meson build system. I do not want a build system to build a build system to build my project, atleast
not at the scale at which I am at right now.
