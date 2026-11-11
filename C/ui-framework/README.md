# ui-framework

C UI playground built on [Leif](https://github.com/cococry/leif) — an immediate-mode, OpenGL UI library.

## Dependencies

| Package | Role |
|---------|------|
| [Leif](https://github.com/cococry/leif) (vendored) | Immediate-mode UI |
| [GLFW](https://www.glfw.org/) | Window + input |
| [cglm](https://github.com/recp/cglm) | Math |
| [libclipboard](https://github.com/jtanx/libclipboard) (vendored) | Clipboard (Leif input) |
| OpenGL 3.3 | Rendering |

## Setup (macOS)

```shell
make setup
```

Installs Homebrew packages (`glfw`, `cglm`, `pkg-config`, `cmake`) and copies Leif default fonts/textures to `~/.leif`.

## Build & run

```shell
make
make run
```

Binary lands at `build/ui-framework`.

## Layout

```
src/main.c          # hello-world Leif app
vendor/leif/        # Leif sources
vendor/libclipboard/# clipboard dependency
Makefile
```

## Notes

- Leif looks for assets under `~/.leif` (fonts + textures). `make` / `make assets` refreshes that directory from `vendor/leif/.leif`.
- Edit `src/main.c` to try widgets (`lf_text`, `lf_button`, `lf_button_fixed`, divs, etc.). See `vendor/leif/include/leif/leif.h` for the API.

## macOS patches

Upstream Leif targets OpenGL 4.5. macOS tops out at OpenGL 4.1, so `vendor/leif/leif.c` is patched locally to:

- replace DSA calls (`glCreate*`, `glTextureParameteri`, `glBindTextureUnit`) with GL 3.3 equivalents
- use GLSL `#version 410 core` instead of `450`
