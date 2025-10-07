mod widgets;
mod window;

use adw::prelude::*;
const APP_ID: &str = "com.example.potato-md";

fn main() -> Result<adw::glib::ExitCode, anyhow::Error> {
    adw::gio::resources_register_include!("potato-md.gresource")?;

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_startup(startup);
    app.connect_activate(build_ui);

    Ok(app.run())
}

fn startup(_app: &adw::Application) {
    adw::gtk::IconTheme::for_display(&adw::gdk::Display::default().unwrap())
        .add_resource_path("/com/example/potato-md/icons");

    // Регистрируем кастомные виджеты
    widgets::InlineView::ensure_type();
}

fn build_ui(app: &adw::Application) {
    let window = window::PotatoWindow::new(app);
    window.present();
}
