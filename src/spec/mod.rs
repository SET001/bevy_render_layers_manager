use bevy::{app::App, prelude::Entity, render::view::RenderLayers};
use rstest::{fixture, rstest};

use crate::{RenderLayerManager, RenderLayersManagerPlugin};

pub mod on_add;
pub mod on_remove;
pub mod on_update;

#[rstest]
fn init_value(app: App) {
  let manager = app.world().resource::<RenderLayerManager>();
  assert_eq!(manager.free_layer, 1);
}

#[fixture]
pub fn app() -> App {
  let mut app = App::new();
  app.add_plugins(RenderLayersManagerPlugin);
  app
}

#[fixture]
pub fn init_app(#[default(vec![])] entities_layers: Vec<Vec<usize>>) -> (App, Vec<Entity>) {
  let mut app = App::new();
  app.add_plugins(RenderLayersManagerPlugin);
  let world = app.world_mut();
  let entities = entities_layers
    .iter()
    .map(|layers| world.spawn(RenderLayers::from_layers(layers)).id())
    .collect::<Vec<Entity>>();
  (app, entities)
}
