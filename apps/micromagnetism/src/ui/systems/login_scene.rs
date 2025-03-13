use crate::ui::{
    events::{ERenderLoginWindow, ERenderSignUpWindow},
    resources::LoginSceneActiveWindow,
};
use bevy::ecs::{event::EventWriter, system::Res};

/// # Login scene
/// This is the UI system that handles drawing UI for the login scene.
pub fn login_scene(
    active_window: Res<LoginSceneActiveWindow>,
    mut login_window_system: EventWriter<ERenderLoginWindow>,
    mut sign_up_window_system: EventWriter<ERenderSignUpWindow>,
) {
    // Fire the event that renders the appropriate window.
    match *active_window {
        LoginSceneActiveWindow::Login => {
            login_window_system.send(ERenderLoginWindow);
        }
        LoginSceneActiveWindow::SignUp => {
            sign_up_window_system.send(ERenderSignUpWindow);
        }
    }
}
