mod window;

use gtk::gio::resources_register_include;
use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

const APP_ID: &str = "io.github.aerphanas";

fn main() -> glib::ExitCode {
    resources_register_include!("src_resources.gresource").expect("Failed to register resources.");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
