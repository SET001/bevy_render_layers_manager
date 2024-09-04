<div align="center">

# `bevy_render_layers_manager`
A [Bevy](https://github.com/bevyengine/bevy) plugin to manage [RenderLayers](https://docs.rs/bevy/latest/bevy/render/view/struct.RenderLayers.html) in app.
</div>

This plugin tracks usage of [RenderLayers](https://docs.rs/bevy/latest/bevy/render/view/struct.RenderLayers.html) in app and provide method to get first free layer for your needs.

## WIP
Currently this is work in progress. While plugin tracks add/remove of RenderLayers components, it still lacks ability to track its updates.

## Usage

This plugin provide `RenderLayerManager` resource with `.get` and `.pick` methods to find out first free render layer in app.

## Bevy Compatibility

| bevy | bevy_render_layers_manager |
|-|-
| 0.14.0 | 0.1.* |
