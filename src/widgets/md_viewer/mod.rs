mod imp;

use adw::gtk::glib;

glib::wrapper! {
    pub struct MdViewer(ObjectSubclass<imp::MdViewer>)
        @extends adw::gtk::Box, adw::gtk::Widget,
        @implements adw::gtk::Accessible, adw::gtk::Buildable, adw::gtk::ConstraintTarget, adw::gtk::Orientable;
}

impl MdViewer {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for MdViewer {
    fn default() -> Self {
        Self::new()
    }
}
