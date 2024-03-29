use crate::game::game_cmps::Game;
use bevy::prelude::*;
use bevy_third_person_camera::{Offset, ThirdPersonCamera};

pub fn spawn(mut cmds: Commands) {
    let translation = Vec3::new(0.0, 1.0, 2.0);

    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            aim_enabled: true,
            aim_zoom: 1.0,
            offset_enabled: true,
            offset_toggle_enabled: true,
            offset_toggle_speed: 8.0,
            offset: Offset::new(0.7, 0.5),
            zoom_enabled: false,
            ..default()
        },
        Name::new("Camera"),
        Game,
    ));
}
