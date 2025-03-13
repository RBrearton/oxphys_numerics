use super::events::*;
use super::resources::{LoginSceneActiveWindow, LoginState, SignUpState};
use crate::is_event;
use crate::ui::systems::system_sets as sets;
use crate::ui::systems::*;
use bevy::ecs::event::EventReader;
use bevy::ecs::schedule::IntoSystemSetConfigs;
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
};

/// # UI plugin
/// This is the core yvium UI plugin.
/// It is responsible for handling all core UI functionality.
/// It doesn't define UI for scenes; that will be handled by the scene-specific plugins.
/// Instead it dictates how UI elements are built and rendered.
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ERenderLoginScene>();
        app.add_event::<ERenderLoginWindow>();
        app.add_event::<ERenderSignUpWindow>();
        app.insert_resource(LoginState::default());
        app.insert_resource(SignUpState::default());
        app.insert_resource(LoginSceneActiveWindow::default());

        // There's quite a lot going on here.
        // First of all, we're separating our systems into sets, which mirrors the
        // hierarchical nature of the UI systems.
        // Secondly, we're specifying when each system should run using run conditions.
        // This is done using the `is_event!` macro, which is a macro that checks if
        // an event has been triggered.
        // Originally I defined a function for each event, but that was really tedious.
        app.add_systems(
            Update,
            (
                (ui_core),
                (login_scene.run_if(is_event!(ERenderLoginScene))).in_set(sets::Scenes),
                (
                    login_window.run_if(is_event!(ERenderLoginWindow)),
                    sign_up_window.run_if(is_event!(ERenderSignUpWindow)),
                )
                    .in_set(sets::LoginSceneWindows),
            ),
        );

        // Now make sure that the systems run in the correct order.
        app.configure_sets(
            Update,
            (
                sets::Scenes.after(ui_core),
                sets::LoginSceneWindows.after(sets::Scenes),
            ),
        );
    }
}
