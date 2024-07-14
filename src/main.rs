mod greet_people;
mod sprite_movement_2d;
mod rendering_shapes;
use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
};

use crate::rendering_shapes::RenderingShapes;
use crate::sprite_movement_2d::SpriteMovement;
//
// fn main() {
//     App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
//         // .add_systems(Startup, add_people)
//         // .add_systems(Update, (hello_world, (update_people, greet_people).chain())).run();
// }


fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(SpriteMovement)
        .add_plugins(RenderingShapes)
        .run();
}


