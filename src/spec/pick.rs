use super::empty_app;
use crate::RenderLayerManager;
use bevy::{app::App, render::view::RenderLayers};
use rstest::rstest;

#[rstest]
fn pick_multiple_layers(mut empty_app: App) {
  let world = empty_app.world_mut();
  let mut manager = world.resource_mut::<RenderLayerManager>();
  let range = 1..=10;
  let layers = range
    .clone()
    .map(|_| manager.pick().clone())
    .collect::<Vec<usize>>();
  layers.iter().for_each(|layer| {
    world.spawn(RenderLayers::layer(*layer));
  });
  empty_app.update();
  let manager = empty_app.world().resource::<RenderLayerManager>();
  assert_eq!(manager.get(), range.max().unwrap() + 1);
}
