NAME = listen-listen

BUILD = build
SRC_DIR = src
GUI_SRC = src/gui
TUI_SRC = src/tui
DAEMON_SRC = src/daemon
COMMON_SRC = src/common
GUI_TARGET = $(BUILD)/listen-gui
TUI_TARGET = $(BUILD)/listen-tui
DAEMON_TARGET = $(BUILD)/listen-daemon
COMMON_TARGET = $(BUILD)/test-common
GUI_TARGET_RELEASE = $(BUILD)/listen-gui-release
OBJ_DIR = $(BUILD)/obj

CC	= gcc
CXX = g++

CFLAGS = -Wall -g
CXXFLAGS = $(CFLAGS) -std=c++17
LDLIBS = `pkg-config --libs gstreamer-1.0 taglib_c taglib`
PKG_FLAGS = `pkg-config --cflags gstreamer-1.0 taglib_c taglib`

.PHONY: all debug

all: $(GUI_TARGET_RELEASE) $(DAEMON_TARGET)
debug: $(GUI_TARGET) $(DAEMON_TARGET) $(COMMON_TARGET)

.PHONY: debug-gui debug-daemon
debug-gui: $(GUI_TARGET)
debug-daemon: $(DAEMON_TARGET)

# ====================================
# Rust builds for the GUI:
# - debug build (for development)
# - release build (for installation)

$(GUI_TARGET_RELEASE):
	@mkdir -p $(@D)
	cargo build --release
	cp target/release/listen-gui $(GUI_TARGET)

$(GUI_TARGET):
	@mkdir -p $(@D)
	cargo build
	cp target/debug/listen-gui $(GUI_TARGET)

$(OBJ_DIR)/%.o: %.c
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -MMD -MP -c $< -o $@ $(PKG_FLAGS)

$(OBJ_DIR)/%.o: %.cpp
	@mkdir -p $(@D)
	$(CXX) $(CXXFLAGS) -MMD -MP -c $< -o $@ $(PKG_FLAGS)

# ====================================
# C and C++ builds for daemon
# DAEMON_SRC is fully written in C
# COMMON_SRC uses taglib (C++) API which is used inside C codebase

D_SRCS = $(shell find $(DAEMON_SRC) -name '*.c' -or -name '*.cpp')
D_OBJS = $(D_SRCS:%.c=$(OBJ_DIR)/%.o)
D_DEPS = $(D_OBJS:.o=.d)

$(DAEMON_TARGET): $(D_OBJS)
	$(CXX) $(D_OBJS) -o $(DAEMON_TARGET) $(LDLIBS)
	@echo "-> built $(DAEMON_TARGET)"

# ====================================
# Common target for testing

C_SRCS = $(shell find $(COMMON_SRC) -name '*.c' -or -name '*.cpp')
C_OBJS = $(C_SRCS:%.c=$(OBJ_DIR)/%.o)
C_DEPS = $(C_OBJS:.o=.d)

$(COMMON_TARGET): $(C_OBJS)
	$(CXX) $(C_OBJS) -o $(COMMON_TARGET) $(LDLIBS)
	@echo "-> built $(COMMON_TARGET)"

-include $(DEPS)

.PHONY: clean
clean:
	rm -rf $(BUILD)
