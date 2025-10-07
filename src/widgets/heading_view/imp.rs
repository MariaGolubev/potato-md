use adw::gtk::glib;
use adw::gtk::prelude::*;
use adw::gtk::subclass::prelude::*;
use std::cell::Cell;

#[derive(Default, glib::Properties)]
#[properties(wrapper_type = super::HeadingView)]
pub struct HeadingView {
    #[property(get, set, minimum = 1, maximum = 6, default = 1)]
    level: Cell<u8>,
}

#[glib::object_subclass]
impl ObjectSubclass for HeadingView {
    const NAME: &'static str = "HeadingView";
    type Type = super::HeadingView;
    type ParentType = crate::widgets::inline_view::InlineView;
}

#[glib::derived_properties]
impl ObjectImpl for HeadingView {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.set_widget_name("heading");
        self.update_css_class();
    }
}

impl WidgetImpl for HeadingView {}

impl HeadingView {
    fn update_css_class(&self) {
        let obj = self.obj();

        // Remove old heading classes
        for i in 1..=6 {
            obj.remove_css_class(&format!("h{}", i));
        }

        // Add new heading class
        let level = self.level.get();
        obj.add_css_class(&format!("h{}", level));
    }
}
