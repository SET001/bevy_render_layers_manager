use super::init_app;
use bevy::{
  app::App,
  prelude::{DespawnRecursiveExt, Entity},
  render::view::RenderLayers,
};
use rstest::rstest;
use rstest_reuse::{self, *};

use crate::RenderLayerManager;

#[template]
#[rstest]
#[case(init_app(vec![vec![0]]), 0, 1)] //	should not remove 0 layer
#[case(init_app(vec![vec![1]]), 0, 1)]
#[case(init_app(vec![vec![2]]), 0, 1)]
#[case(init_app(vec![vec![1], vec![2], vec![3]]), 1, 2)]
#[case(init_app(vec![vec![1], vec![2], vec![3]]), 0, 1)]
#[case(init_app(vec![vec![1], vec![2], vec![9], vec![10]]), 2, 3)]
#[case(init_app(vec![vec![1, 2], vec![2]]), 1, 3)] //	should check if removed layer used elsewhere
#[case(init_app(vec![vec![1, 2], vec![2]]), 0, 1)]

fn remove_cases(#[case] input: (usize, usize), #[case] to_remove: usize, #[case] expect: usize) {}

#[apply(remove_cases)]
fn when_component_removed(input: (App, Vec<Entity>), to_remove: usize, expect: usize) {
  let (mut app, entities) = input;
  app
    .world_mut()
    .entity_mut(*entities.get(to_remove).unwrap())
    .remove::<RenderLayers>();
  app.update();
  let manager = app.world().resource::<RenderLayerManager>();
  assert_eq!(manager.free_layer, expect);
}

#[apply(remove_cases)]
fn when_entity_despawned(input: (App, Vec<Entity>), to_remove: usize, expect: usize) {
  let (mut app, entities) = input;
  app
    .world_mut()
    .entity_mut(*entities.get(to_remove).unwrap())
    .despawn_recursive();
  app.update();
  let manager = app.world().resource::<RenderLayerManager>();
  assert_eq!(manager.free_layer, expect);
}
