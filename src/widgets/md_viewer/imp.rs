use adw::gtk::glib;
use adw::gtk::prelude::*;
use adw::gtk::subclass::prelude::*;

#[derive(Default)]
pub struct MdViewer {}

#[glib::object_subclass]
impl ObjectSubclass for MdViewer {
    const NAME: &'static str = "MdViewer";
    type Type = super::MdViewer;
    type ParentType = adw::gtk::Box;
}

impl ObjectImpl for MdViewer {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.set_orientation(adw::gtk::Orientation::Vertical);
        obj.set_spacing(0);
    }
}

impl WidgetImpl for MdViewer {}
impl BoxImpl for MdViewer {}
