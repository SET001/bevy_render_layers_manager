use super::init_app;
use crate::RenderLayerManager;
use bevy::render::view::RenderLayers;
use rstest::rstest;
use rstest_reuse::{self, *};

#[template]
#[rstest]
// #[case(vec![(vec![1], vec![2])], 1)]
#[case(vec![(vec![1, 2, 3, 4], vec![1, 3, 4])], 2)]
fn update_cases(
  #[case] entity_layers_updates: Vec<(Vec<usize>, Vec<usize>)>,
  #[case] expect: usize,
) {
}

#[apply(update_cases)]
fn when_owerwritten(entity_layers_updates: Vec<(Vec<usize>, Vec<usize>)>, expect: usize) {
  let entity_layers = entity_layers_updates
    .clone()
    .into_iter()
    .map(|(layers, _)| layers)
    .collect::<Vec<Vec<usize>>>();
  let (mut app, entities) = init_app(entity_layers);
  app.update();
  println!("sending updates now");
  for (i, entity) in entities.iter().enumerate() {
    let updates = entity_layers_updates.get(i).unwrap().1.clone();
    println!("Inserting new renderLayers value {:?}", updates);
    app
      .world_mut()
      .get_entity_mut(*entity)
      .unwrap()
      .insert(RenderLayers::from_layers(&updates));
  }
  app.update();
  app.update();
  let manager = app.world_mut().resource_mut::<RenderLayerManager>();
  assert_eq!(manager.free_layer, expect);
}

// #[apply(update_cases)]
// fn when_modified(entity_layers_updates: Vec<(Vec<usize>, Vec<usize>)>, expect: usize) {
//   use crate::RenderLayerManager;

//   let entity_layers = entity_layers_updates
//     .clone()
//     .into_iter()
//     .map(|(layers, _)| layers)
//     .collect::<Vec<Vec<usize>>>();
//   let (mut app, entities) = init_app(entity_layers);
//   app.update();
//   for (i, entity) in entities.iter().enumerate() {
//     let updates = entity_layers_updates.get(i).unwrap().1.clone();
//     let mut layers = app.world_mut().get_mut::<RenderLayers>(*entity).unwrap();
//     let updates = entity_layers_updates.get(i).unwrap().1.clone();
//     *layers = RenderLayers::from_layers(&updates);
//   }
//   app.update();
//   let manager = app.world_mut().resource_mut::<RenderLayerManager>();
//   assert_eq!(manager.free_layer, expect);
// }
