CC    = gcc
CFLAG = -Wall -g
BUILD = build
GSTRF = `pkg-config --cflags --libs gstreamer-1.0 taglib_c`

FILES_DAEMON = src/daemon/utils.c src/daemon/controller.c src/daemon/server.c src/daemon/main.c
FILES_GUI = src/gui/main.c

build-gui:
	cargo build && \
		cp target/debug/listen-gui $(BUILD)/listen-gui && \
		$(BUILD)/listen-gui

build-gui-release:
	cargo build --release && \
		cp target/release/listen-gui $(BUILD)/listen-gui && \
		$(BUILD)/listen-gui
build-server:
	$(CC) $(CFLAG) $(FILES_DAEMON) -o $(BUILD)/gst-server $(GSTRF) && $(BUILD)/gst-server

test1:
	$(CC) $(CFLAG) tests/test.c -o $(BUILD)/test1 $(GSTRF) && $(BUILD)/test1

PROG_FLAGS =
test2:
	$(CC) $(CFLAG) src/common/common.c -o $(BUILD)/test2 $(GSTRF) && $(BUILD)/test2 $(PROG_FLAGS)

clean:
	rm -r $(BUILD)
	mkdir $(BUILD)
	touch $(BUILD)/.gitkeep

.PHONY: test clean
