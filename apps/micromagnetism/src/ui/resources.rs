use bevy::ecs::system::Resource;

/// # Login state
/// State to store login form data.
#[derive(Resource, Default)]
pub(crate) struct LoginState {
    pub(crate) email: String,
    pub(crate) password: String,
}

/// # Sign up state
/// State to store sign up form data.
#[derive(Resource, Default)]
pub(crate) struct SignUpState {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) confirm_password: String,
}

/// # Login scene active window
/// This is used to determine which window to show in the login scene.
#[derive(Resource, Default)]
pub(crate) enum LoginSceneActiveWindow {
    #[default]
    Login,
    SignUp,
}
