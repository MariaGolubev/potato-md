use adw::prelude::*;

const APP_ID: &str = "com.example.PotatoMD";

fn main() -> Result<adw::glib::ExitCode, anyhow::Error> {
    adw::gio::resources_register_include!("patato-md.gresource")?;

    let app = adw::Application::builder().application_id(APP_ID).build();

    Ok(app.run())
}
