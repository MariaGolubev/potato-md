mod imp;

use adw::gtk::glib;

glib::wrapper! {
    pub struct HeadingView(ObjectSubclass<imp::HeadingView>)
        @extends crate::widgets::inline_view::InlineView, adw::gtk::Widget,
        @implements adw::gtk::Accessible, adw::gtk::Buildable, adw::gtk::ConstraintTarget;
}

impl HeadingView {
    pub fn new(level: u8) -> Self {
        glib::Object::builder()
            .property("level", level)
            .build()
    }
}
