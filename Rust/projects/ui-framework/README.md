# ui-framework

Immediate-mode UI framework from scratch on **GLFW + OpenGL 3.3**.

## Status

Phase 1 — window shell: GLFW window, GL clear loop, Escape to quit.

## Dependencies

- [GLFW](https://www.glfw.org/) (system library; `brew install glfw` on macOS)
- `glfw` crate — window + input
- `gl` crate — OpenGL function loading

## Run

```shell
cargo run
```

## Roadmap

1. Shell — window, clear, Esc *(done)*
2. Draw — colored rects, then text
3. Input — mouse / keyboard → hit-testing
4. Widgets — text, button, …
5. Layout — row / column + padding
6. Frame API — `begin` / `end`
