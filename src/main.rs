use adw::prelude::*;
use adw::{Application, ApplicationWindow};

mod preferences;
mod ui;

fn main() {
    let application = Application::builder()
        .application_id("com.zendard.MBMM")
        .build();

    application.connect_activate(|app| {
        let content = ui::content();

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
