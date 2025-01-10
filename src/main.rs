use gtk::{glib, prelude::*, Application, ApplicationWindow};

const APP_ID: &str = "com.fekoneko.ppv.app";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pixiv Powerful Viewer")
        .build();

    window.present();
}
