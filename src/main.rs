mod lib;
mod widgets;

use adw::prelude::*;
use adw::{gio, glib, Application};
use widgets::application_window::ApplicationWindow;

const APP_ID: &str = "com.fekoneko.ppv.app";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("ppv.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| ApplicationWindow::new(app).present());

    app.run()
}
