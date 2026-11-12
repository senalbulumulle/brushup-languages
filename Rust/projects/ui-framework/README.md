# ui-framework

Immediate-mode UI framework from scratch on **GLFW + OpenGL 3.3**.

Right now this is a window shell: an 800×600 GLFW window, a dark GL clear each frame, and Escape to quit.

## Layout

```
src/
  main.rs     # demo loop
  lib.rs      # crate root
  window.rs   # GLFW lifecycle, events, viewport
  render.rs   # framebuffer clear
  input.rs    # (empty)
  ui.rs       # (empty)
```

## Dependencies

- [GLFW](https://www.glfw.org/) (system library; `brew install glfw` on macOS)
- `glfw` — window + input
- `gl` — OpenGL function loading

## Run

```shell
cargo run
```
