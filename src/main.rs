mod widgets;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use widgets::window::Window;

const APP_ID: &str = "com.fekoneko.ppv.app";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("ppv.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| Window::new(app).present());

    app.run()
}
