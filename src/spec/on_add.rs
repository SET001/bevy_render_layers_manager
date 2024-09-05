use super::init_app;
use crate::RenderLayerManager;
use bevy::{prelude::Entity, render::view::RenderLayers};
use rstest::rstest;
use rstest_reuse::{self, *};

#[template]
#[rstest]
#[case(vec![vec![0]], 1)]
#[case(vec![vec![1]], 2)]
#[case(vec![vec![2]], 1)]
#[case(vec![vec![2, 3]], 1)]
#[case(vec![vec![1, 2]], 3)]
#[case(vec![vec![1, 3, 4]], 2)]
#[case(vec![vec![1, 2, 3]], 4)]
#[case(vec![vec![1, 2, 0]], 3)]
#[case(vec![vec![1, 2], vec![2]], 3)]
#[case(vec![vec![1, 2], vec![4, 5]], 3)]
#[case(vec![vec![1, 2], vec![3, 4]], 5)]
#[case(vec![vec![1, 2], vec![2, 3]], 4)]
fn add_cases(#[case] entities_layers: Vec<Vec<usize>>, #[case] expect: usize) {}

#[apply(add_cases)]
fn when_spawning(entities_layers: Vec<Vec<usize>>, expect: usize) {
  let (mut app, _) = init_app(entities_layers);
  let manager = app.world_mut().resource_mut::<RenderLayerManager>();
  assert_eq!(manager.free_layer, expect);
}

#[apply(add_cases)]
fn when_inserting(entities_layers: Vec<Vec<usize>>, expect: usize) {
  let (mut app, _) = init_app(vec![]);
  let entities = entities_layers
    .iter()
    .map(|layers| (app.world_mut().spawn_empty().id(), layers))
    .collect::<Vec<(Entity, &Vec<usize>)>>();
  app.update();
  entities.iter().for_each(|(entity, layers)| {
    app
      .world_mut()
      .get_entity_mut(*entity)
      .unwrap()
      .insert(RenderLayers::from_layers(&layers));
  });
  let manager = app.world_mut().resource_mut::<RenderLayerManager>();
  assert_eq!(manager.free_layer, expect);
}
