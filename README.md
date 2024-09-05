<div align="center">

# `bevy_render_layers_manager`
A [Bevy](https://github.com/bevyengine/bevy) plugin to manage [RenderLayers](https://docs.rs/bevy/latest/bevy/render/view/struct.RenderLayers.html) in app.

[![crates.io](https://img.shields.io/crates/v/bevy_render_layers_manager)](https://crates.io/crates/bevy_render_layers_manager)
[![crates.io](https://img.shields.io/crates/d/bevy_render_layers_manager)](https://crates.io/crates/bevy_render_layers_manager)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
</div>

This plugin tracks usage of [RenderLayers](https://docs.rs/bevy/latest/bevy/render/view/struct.RenderLayers.html) in app and provide method to get first free layer for your needs.

## Usage

Add `RenderLayersManagerPlugin` in your app. This plugin adds `RenderLayerManager` resource with `.get` method to get first free layer.

You can also use `.pick` method that will return free layer and mark it as used.

## Example

```rust
use bevy::prelude::*;
use bevy_rander_layer_manager::*;

fn main() {
  let mut app = App::new();
  app
    .add_plugins(RenderLayersManagerPlugin)
    .add_systems(Startup, startup);
  app.run();
}

fn startup(layers_manager: Res<RenderLayerManager>) {
  println!("Current free layer is: {}", layers_manager.get());
}
```

## Bevy Compatibility

| bevy | bevy_render_layers_manager |
|-|-
| 0.14.0 | 0.1.* |
