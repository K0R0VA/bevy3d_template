use bevy::prelude::*;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::{DebugRenderMode, RapierDebugRenderPlugin}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin {
            mode: DebugRenderMode::all(),
            ..Default::default()
        })
        .run();
}