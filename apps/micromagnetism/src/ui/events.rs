use bevy::ecs::event::Event;

/// # Render login scene event
/// Triggering this event will force the login scene to be rendered.
#[derive(Event)]
pub struct ERenderLoginScene;

/// # Render login window event
/// Triggering this event forces the login window to be displayed.
#[derive(Event)]
pub struct ERenderLoginWindow;

/// # Render sign up window event
/// Triggering this event forces the sign up window to be displayed.
#[derive(Event)]
pub struct ERenderSignUpWindow;
