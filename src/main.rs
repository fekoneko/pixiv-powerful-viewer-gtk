mod widgets;

use gtk::{gio, glib, prelude::*, Application};
use widgets::window::Window;

const APP_ID: &str = "com.fekoneko.ppv.app";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("ppv.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
