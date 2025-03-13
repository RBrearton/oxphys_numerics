use bevy::ecs::system::ResMut;
use bevy_egui::EguiContexts;

use crate::ui::resources::{LoginSceneActiveWindow, LoginState};

/// # Login window system
/// This should only run when the main UI system triggers the appropriate event.
/// As a result, we don't check any state to make sure that this should run.
/// We assume that, if this system is triggered, it should run.
pub fn login_window(
    mut contexts: EguiContexts,
    mut login_state: ResMut<LoginState>,
    mut active_window: ResMut<LoginSceneActiveWindow>,
) {
    // Set up the actual window.
    let login_window = egui::Window::new("Login")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .resizable(false)
        .collapsible(false);

    // This is where the logic lives that defines the UI that sits inside the window.
    login_window.show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            // Title.
            ui.heading("Welcome back!");
            ui.add_space(16.0);

            // Email field.
            ui.label("Email:");
            ui.text_edit_singleline(&mut login_state.email);
            ui.add_space(8.0);

            // Password field.
            ui.label("Password:");
            ui.add(
                egui::TextEdit::singleline(&mut login_state.password).password(true),
            );
            ui.add_space(16.0);

            // Login button.
            if ui.button("Login").clicked() {
                // TODO: Add login logic here.
                println!("Login attempted with email: {}", login_state.email);
            }

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label("Don't have an account?");
                if ui.link("Sign Up").clicked() {
                    *active_window = LoginSceneActiveWindow::SignUp;
                }
            });
        });
    });
}
