# egui-gizmo

[![Latest version](https://img.shields.io/crates/v/egui-gizmo.svg)](https://crates.io/crates/egui-gizmo)
[![Documentation](https://docs.rs/egui-gizmo/badge.svg)](https://docs.rs/egui-gizmo)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

3d transformation gizmo built on top of the [egui](https://github.com/emilk/egui) library.

[Try it out in a web demo](https://urholaukkarinen.github.io/egui-gizmo/)

![Rotation](media/rotation.png)
![Translation](media/translation.png)
![Scale](media/scale.png)

## Usage

```rust
let gizmo = Gizmo::new("My gizmo")
    .view_matrix(view_matrix)
    .projection_matrix(projection_matrix)
    .model_matrix(model_matrix)
    .mode(GizmoMode::Rotate);

if let Some(response) = gizmo.interact(ui) {
    model_matrix = response.transform();
}
```

For examples, see the [examples directory](examples/).

The gizmo exposes matrices and vectors as [mint](https://github.com/kvark/mint) types, which means it is easy to use with matrix types from various crates
such as [nalgebra](https://github.com/dimforge/nalgebra), [glam](https://github.com/bitshifter/glam-rs)
and [cgmath](https://github.com/rustgd/cgmath). You may need to enable a `mint` feature, depending on the math library.

## Integration Guide

For detailed instructions on how to integrate and use this library in your own Rust project, especially in a game engine, please refer to the [integration documentation](DOCUMENTACAO_USO.md).

## Project Status

This project has been updated and improved to work with modern Rust toolchains and dependencies. The main library is stable and functional, with examples demonstrating its usage.

## Contributing

Feel free to contribute to this project by submitting issues or pull requests. Check the repository at [https://github.com/dumestre/Eguizmo](https://github.com/dumestre/Eguizmo) for the latest version and contribution guidelines.
