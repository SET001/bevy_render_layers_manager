use bevy::{
  app::{Plugin, Update},
  prelude::*,
  render::view::RenderLayers,
  utils::HashMap,
};
#[cfg(test)]
mod spec;

#[derive(Resource)]
pub struct RenderLayerManager {
  used_layers: HashMap<usize, usize>,
  free_layer: usize,
}

impl RenderLayerManager {
  //	Return nearest (to zero) free render layer
  pub fn get(&self) -> usize {
    self.free_layer
  }

  //	Return nearest (to zero) free render layer and mark it as used
  pub fn pick(&mut self) -> usize {
    let free_layer = self.free_layer;
    self.add(self.free_layer, 0);
    free_layer
  }

  //	Mark render layer as used
  fn add(&mut self, layer: usize, init_num: usize) {
    match self.used_layers.get_mut(&layer) {
      Some(num) => {
        *num += 1;
      }
      None => {
        self.used_layers.insert(layer, init_num);
        if layer == self.free_layer {
          while self.used_layers.get(&self.free_layer).is_some() {
            self.free_layer += 1;
          }
        }
      }
    };
  }

  //	Unmark render layer as used
  fn remove(&mut self, layer: usize) {
    if let Some(num) = self.used_layers.get_mut(&layer) {
      if *num == 1 && layer != 0 {
        self.used_layers.remove(&layer);
        if layer < self.free_layer {
          self.free_layer = layer;
        }
      } else {
        *num -= 1;
      }
    }
  }
}

impl Default for RenderLayerManager {
  fn default() -> Self {
    Self {
      used_layers: HashMap::from([(0, 0)]),
      free_layer: 1,
    }
  }
}

fn on_add(
  trigger: Trigger<OnAdd, RenderLayers>,
  q_render_layers: Query<&RenderLayers>,
  mut layers_manager: ResMut<RenderLayerManager>,
) {
  q_render_layers
    .get(trigger.entity())
    .unwrap()
    .iter()
    .for_each(|layer| layers_manager.add(layer, 1));
}

fn on_remove(
  trigger: Trigger<OnRemove, RenderLayers>,
  q_render_layers: Query<&RenderLayers>,
  mut layers_manager: ResMut<RenderLayerManager>,
) {
  q_render_layers
    .get(trigger.entity())
    .unwrap()
    .iter()
    .for_each(|layer| layers_manager.remove(layer));
}

#[derive(Component, Debug)]
pub struct Old<T: Component>(T);

fn on_update(
  mut commands: Commands,
  query: Query<(Entity, Ref<RenderLayers>, Option<&Old<RenderLayers>>)>,
  mut layers_manager: ResMut<RenderLayerManager>,
) {
  for (entity, layers, maybe_old) in query.iter() {
    if layers.is_changed() {
      if let Some(old) = maybe_old {
        let union = layers.union(&old.0);

        union
          .symmetric_difference(&layers)
          .iter()
          .for_each(|layer| layers_manager.remove(layer));
        union
          .symmetric_difference(&old.0)
          .iter()
          .for_each(|layer| layers_manager.add(layer, 1));
      }
    }
    commands.entity(entity).insert(Old(layers.clone()));
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
