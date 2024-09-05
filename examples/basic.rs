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
