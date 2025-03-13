use bevy::ecs::schedule::SystemSet;

/// # Scene system set
/// All systems that orchestrate the rendering of UI for entire scenes must be a
/// member of this system set.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scenes;

/// # Login scene windows system set
/// All systems involved in rendering login scene UI must be a member of this system
/// set.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoginSceneWindows;
