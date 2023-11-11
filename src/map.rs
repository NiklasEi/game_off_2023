use crate::GameState;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::{Collider, RigidBody};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_map);
    }
}

fn spawn_map(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(640.0, 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -180.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(640., 2.),
    ));
}
