use crate::preferences::Preferences;
use adw::prelude::*;
use adw::{Application, ApplicationWindow};

mod manage;
mod preferences;
mod ui;

fn main() {
    let preferences = Preferences::load();

    let application = Application::builder()
        .application_id("com.zendard.MBMM")
        .build();

    application.connect_activate(move |app| {
        let content = ui::content(preferences.clone());

        let window = ApplicationWindow::builder()
            .application(app)
            .title("MBMM")
            // add content to window
            .content(&content)
            .build();
        window.present();
    });

    application.run();
}
