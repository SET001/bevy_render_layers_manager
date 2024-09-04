use bevy::{
  app::{Plugin, Update},
  prelude::{Changed, Entity, OnAdd, OnRemove, Query, ResMut, Resource, Trigger},
  render::view::RenderLayers,
  utils::HashSet,
};
#[cfg(test)]
mod spec;

#[derive(Resource)]
pub struct RenderLayerManager {
  used_layers: HashSet<usize>,
  free_layer: usize,
}

impl RenderLayerManager {
  //	Return nearest (to zero) free render layer
  pub fn get(&self) -> RenderLayers {
    RenderLayers::none().with(self.free_layer)
  }

  //	Return nearest (to zero) free render layer and mark it as used
  pub fn pick(&mut self) -> RenderLayers {
    let layers = RenderLayers::none().with(self.free_layer);
    self.add(&layers);
    layers
  }

  //	Mark render layers as used
  pub fn add(&mut self, layers: &RenderLayers) {
    layers.iter().for_each(|layer| {
      if layer > 0 && !self.used_layers.contains(&layer) {
        self.used_layers.insert(layer);
        if layer == self.free_layer {
          while self.used_layers.contains(&self.free_layer) {
            self.free_layer += 1;
          }
        }
      }
    });
  }

  //	Unmark render layers as used
  pub fn remove(&mut self, layers: &RenderLayers) {
    layers.iter().for_each(|layer| {
      if layer > 0 {
        self.used_layers.remove(&layer);
        if layer < self.free_layer {
          self.free_layer = layer;
        }
      }
    });
  }
}

impl Default for RenderLayerManager {
  fn default() -> Self {
    Self {
      used_layers: HashSet::from([0]),
      free_layer: 1,
    }
  }
}

fn on_add(
  trigger: Trigger<OnAdd, RenderLayers>,
  q_render_layers: Query<&RenderLayers>,
  mut layers_manager: ResMut<RenderLayerManager>,
) {
  let layers = q_render_layers.get(trigger.entity()).unwrap();
  layers_manager.add(layers);
}

fn on_remove(
  trigger: Trigger<OnRemove, RenderLayers>,
  q_render_layers: Query<&RenderLayers>,
  mut layers_manager: ResMut<RenderLayerManager>,
) {
  let all_render_layers = q_render_layers.iter().collect::<Vec<&RenderLayers>>();
  q_render_layers
    .get(trigger.entity())
    .unwrap()
    .iter()
    .for_each(|layer| {
      let layer = &RenderLayers::none().with(layer);
      if all_render_layers
        .iter()
        .filter(|render_layers| render_layers.intersects(layer))
        .count()
        < 2
      {
        layers_manager.remove(layer);
      }
    });
}

fn on_update(query: Query<(Entity, &RenderLayers), Changed<RenderLayers>>) {
  for (entity, layers) in query.iter() {
    // println!("render layers changed from {} to {}",)
  }
}

pub struct RenderLayersManagerPlugin;
impl Plugin for RenderLayersManagerPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app
      .init_resource::<RenderLayerManager>()
      .add_systems(Update, on_update)
      .observe(on_add)
      .observe(on_remove);
  }
}
