use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{gamepad::gamepad_rcs::MyGamepad, AppState};

use super::{game_cmps::*, game_evs::*, game_res::*};

pub fn exit_game(
    btns: Res<Input<GamepadButton>>,
    keys: Res<Input<KeyCode>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_time: ResMut<GameTime>,
) {
    // gamepad
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::Start)))
        .unwrap_or(false);

    if gamepad_input || keys.just_pressed(KeyCode::Escape) {
        next_app_state.set(AppState::MainMenu);
        game_time.0.reset(); // reset stopwatch
    }
}

/// Change state to GameOver when GameOver event is fired
pub fn game_over(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_over_evr: EventReader<GameOver>,
) {
    for _ev in game_over_evr.iter() {
        next_app_state.set(AppState::GameOver);
    }
}

/// despawn all entities with a game component when exiting AppState::Game
pub fn despawn_game(mut cmds: Commands, all_q: Query<Entity, With<Game>>) {
    for ent in all_q.iter() {
        cmds.entity(ent).despawn_recursive();
    }
}

/// un-hides the cursor when exiting game state
pub fn show_cursor(mut window_q: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_q.get_single_mut().unwrap();
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}
