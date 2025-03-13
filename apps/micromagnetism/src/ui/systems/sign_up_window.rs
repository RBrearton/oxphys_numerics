use bevy::ecs::system::ResMut;
use bevy_egui::EguiContexts;

use crate::ui::resources::{LoginSceneActiveWindow, SignUpState};

/// # Sign up window system
/// Running this system triggers the rendering of the sign up window.
pub fn sign_up_window(
    mut contexts: EguiContexts,
    mut sign_up_state: ResMut<SignUpState>,
    mut active_window: ResMut<LoginSceneActiveWindow>,
) {
    // Set up the actual window that we're going to draw in.
    let window = egui::Window::new("Sign up")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .resizable(false)
        .collapsible(false);

    // This is where the logic lives that defines the UI that sits inside the window.
    window.show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            // Title.
            ui.heading("Create account");
            ui.add_space(16.0);

            // Username field.
            ui.label("Username:");
            ui.text_edit_singleline(&mut sign_up_state.username);
            ui.add_space(8.0);

            // Email field.
            ui.label("Email:");
            ui.text_edit_singleline(&mut sign_up_state.email);
            ui.add_space(8.0);

            // Password field.
            ui.label("Password:");
            ui.add(
                egui::TextEdit::singleline(&mut sign_up_state.password).password(true),
            );
            ui.add_space(8.0);

            // Confirm password field.
            ui.label("Confirm password:");
            ui.add(
                egui::TextEdit::singleline(&mut sign_up_state.confirm_password)
                    .password(true),
            );
            ui.add_space(16.0);

            // Sign up button.
            if ui.button("Create account").clicked() {
                // TODO: Add sign up logic here.
                println!(
                    "Sign up attempted with username: {}",
                    sign_up_state.username
                );
            }

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Already have an account?");
                if ui.link("Login").clicked() {
                    *active_window = LoginSceneActiveWindow::Login;
                }
            });
        });
    });
}
