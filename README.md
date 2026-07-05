# macroquad_layouts

A learning project. My main goal is to get better at writing Rust, using
this as an excuse to figure out how to build nestable UI layouts on top of
[macroquad](https://github.com/not-fl3/macroquad).

The idea: a small library where components (buttons, layouts, etc.) can be
nested inside each other, similar to how layouts work in Flutter/web CSS.

![screenshot](src/assets/image.png)

## Status

MVP stage. Currently working:
- Layouts creation.
- Layouts can nest other layouts.
- `HorizontalLayout` and `VerticalLayout` containers that can nest components
- A test `Button` component

Buttons don't react to events yet — that's next.

## Running

```
cargo run
```
