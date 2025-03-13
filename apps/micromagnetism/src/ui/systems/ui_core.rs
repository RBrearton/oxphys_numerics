use crate::{scene::Scene, ui::events::ERenderLoginScene};
use bevy::ecs::{event::EventWriter, system::Res};

/// # UI core
/// This is the core UI system that handles all UI in the yvium client.
/// The way that this works is it orchestrates sub-systems that handle individual
/// windows.
pub fn ui_core(
    active_scene: Res<Scene>,
    mut login_window_system: EventWriter<ERenderLoginScene>,
) {
    // Fire the appropriate event based on the active scene.
    match *active_scene {
        Scene::Login => login_window_system.send(ERenderLoginScene),
    };
}
