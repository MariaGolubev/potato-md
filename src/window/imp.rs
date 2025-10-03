use adw::glib;
use adw::glib::subclass::InitializingObject;
use adw::gtk;
use adw::subclass::prelude::*;

#[derive(Default, gtk4_macros::CompositeTemplate)]
#[template(resource = "/com/example/potato-md/ui/window.ui")]
pub struct PotatoWindow {
    #[template_child]
    status_page: TemplateChild<adw::StatusPage>,
}

#[glib::object_subclass]
impl ObjectSubclass for PotatoWindow {
    const NAME: &'static str = "PotatoWindow";
    type Type = super::PotatoWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for PotatoWindow {}

// Trait shared by all widgets
impl WidgetImpl for PotatoWindow {}

// Trait shared by all windows
impl WindowImpl for PotatoWindow {}

impl ApplicationWindowImpl for PotatoWindow {}

// Trait shared by all application windows
impl AdwApplicationWindowImpl for PotatoWindow {}
