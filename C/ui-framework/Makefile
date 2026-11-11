APP      := build/ui-framework
CC       := clang
ROOT     := $(abspath .)

LEIF_DIR := $(ROOT)/vendor/leif
CLIP_DIR := $(ROOT)/vendor/libclipboard
CLIP_BUILD := $(CLIP_DIR)/build
CLIP_PREFIX := $(CLIP_DIR)/prefix

PKG_CFLAGS := $(shell pkg-config --cflags glfw3 cglm)
PKG_LIBS   := $(shell pkg-config --libs glfw3 cglm)

LEIF_INCS := \
	-I$(LEIF_DIR)/include \
	-I$(LEIF_DIR)/vendor/glad/include \
	-I$(LEIF_DIR)/vendor/stb_image \
	-I$(LEIF_DIR)/vendor/stb_image_resize \
	-I$(LEIF_DIR)/vendor/stb_truetype \
	-I$(CLIP_PREFIX)/include

CFLAGS  := -std=c11 -Wall -Wextra -O2 -DLF_GLFW $(PKG_CFLAGS) $(LEIF_INCS)
LDFLAGS := $(PKG_LIBS) -L$(CLIP_PREFIX)/lib -lclipboard

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
	LDFLAGS += -framework OpenGL -framework Cocoa -framework IOKit -framework CoreFoundation -framework AppKit
else
	LDFLAGS += -lGL -ldl -lpthread
endif

.PHONY: all clean run setup assets

all: assets $(APP)

$(APP): build/main.o build/leif.o build/glad.o $(CLIP_PREFIX)/lib/libclipboard.a | build
	$(CC) $^ -o $@ $(LDFLAGS)

build/main.o: src/main.c $(CLIP_PREFIX)/lib/libclipboard.a | build
	$(CC) $(CFLAGS) -c $< -o $@

build/leif.o: $(LEIF_DIR)/leif.c $(CLIP_PREFIX)/lib/libclipboard.a | build
	$(CC) $(CFLAGS) -c $< -o $@

build/glad.o: $(LEIF_DIR)/vendor/glad/src/glad.c | build
	$(CC) $(CFLAGS) -c $< -o $@

$(CLIP_PREFIX)/lib/libclipboard.a:
	cmake -S $(CLIP_DIR) -B $(CLIP_BUILD) \
		-DCMAKE_POLICY_VERSION_MINIMUM=3.5 \
		-DCMAKE_INSTALL_PREFIX=$(CLIP_PREFIX) \
		-DBUILD_SHARED_LIBS=OFF
	cmake --build $(CLIP_BUILD) --target clipboard -j
	cmake --install $(CLIP_BUILD)

build:
	mkdir -p build

assets:
	mkdir -p $(HOME)/.leif
	cp -R $(LEIF_DIR)/.leif/. $(HOME)/.leif/

setup:
	brew install glfw cglm pkg-config cmake
	$(MAKE) assets

run: $(APP)
	./$(APP)

clean:
	rm -rf build $(CLIP_BUILD) $(CLIP_PREFIX)
