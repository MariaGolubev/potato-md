mod buffer;
mod imp;

use adw::gtk::glib;
// use adw::gtk::subclass::prelude::*;

pub use buffer::{InlineAnchor, InlineBuffer, InlinePos, TextAttr};

glib::wrapper! {
    pub struct InlineView(ObjectSubclass<imp::InlineView>)
        @extends adw::gtk::Widget,
        @implements adw::gtk::Accessible, adw::gtk::Buildable, adw::gtk::ConstraintTarget;
}

impl InlineView {
    /// Create a new InlineView widget
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    // /// Create a new InlineView with a specific buffer
    // pub fn with_buffer(buffer: &InlineBuffer) -> Self {
    //     let view: Self = glib::Object::builder().build();
    //     view.set_buffer(Some(buffer));
    //     view
    // }

    // /// Get the buffer used by this view
    // pub fn buffer(&self) -> Option<InlineBuffer> {
    //     self.imp().buffer()
    // }

    // /// Set the buffer for this view
    // pub fn set_buffer(&self, buffer: Option<&InlineBuffer>) {
    //     self.imp().set_buffer(buffer.cloned());
    // }

    // /// Get the current text (convenience method)
    // pub fn text(&self) -> String {
    //     self.imp().text()
    // }

    // /// Set the text (convenience method - creates/updates buffer)
    // pub fn set_text(&self, text: &str) {
    //     self.imp().set_text(text);
    // }
}

impl Default for InlineView {
    fn default() -> Self {
        Self::new()
    }
}
