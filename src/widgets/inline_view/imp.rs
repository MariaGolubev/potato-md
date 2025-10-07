use std::cell::RefCell;

use adw::gtk::glib;
use adw::gtk::pango::{self, SCALE};
use adw::gtk::prelude::*;
use adw::gtk::subclass::prelude::*;
use glib_macros::Properties;

use super::buffer::InlineBuffer;

#[derive(Default, Properties)]
#[properties(wrapper_type = super::InlineView)]
pub struct InlineView {
    #[property(get, set = Self::set_buffer, nullable)]
    #[property(name = "text", get = Self::get_text, set = Self::set_text, type = String)]
    buffer: RefCell<Option<InlineBuffer>>,
    buffer_signal_id: RefCell<Option<glib::SignalHandlerId>>,
    needs_update: RefCell<bool>,
    layout: RefCell<Option<pango::Layout>>,
}

impl InlineView {
    fn set_buffer(&self, buffer: Option<InlineBuffer>) {
        // Disconnect previous signal if exists
        if let Some(signal_id) = self.buffer_signal_id.borrow_mut().take() {
            if let Some(old_buffer) = self.buffer.borrow().as_ref() {
                old_buffer.disconnect(signal_id);
            }
        }

        // Connect to new buffer's changed signal
        if let Some(ref buf) = buffer {
            let signal_id = buf.connect_local(
                "changed",
                false,
                glib_macros::clone!(
                    #[weak(rename_to = view)]
                    self,
                    #[upgrade_or]
                    None,
                    move |_| {
                        view.needs_update.replace(true);
                        view.obj().queue_resize();
                        None
                    }
                ),
            );
            self.buffer_signal_id.replace(Some(signal_id));
        }

        self.buffer.replace(buffer);
        self.needs_update.replace(true);
    }

    fn get_text(&self) -> String {
        self.buffer
            .borrow()
            .as_ref()
            .map(|b| b.text())
            .unwrap_or_default()
    }

    fn set_text(&self, text: &str) {
        if let Some(buffer) = self.buffer.borrow().as_ref() {
            buffer.set_text(text);
            self.needs_update.replace(true);
        } else {
            let buffer = InlineBuffer::new();
            buffer.set_text(text);
            self.buffer.replace(Some(buffer));
            self.needs_update.replace(true);
        }
    }

    fn rebuild_layout(&self) {
        if let Some(buffer) = self.buffer.borrow().as_ref() {
            let layout = self.obj().create_pango_layout(Some(&buffer.text()));
            layout.set_wrap(pango::WrapMode::WordChar);

            // Apply attributes from buffer
            let attr_list = buffer.build_pango_attributes();
            layout.set_attributes(Some(&attr_list));

            self.layout.borrow_mut().replace(layout);
        } else {
            self.layout.borrow_mut().take();
        }
    }

    fn update_layout(&self, orientation: adw::gtk::Orientation, for_size: i32) -> (i32, i32) {
        if self.needs_update.replace(false) {
            self.rebuild_layout();
        }

        if let Some(layout) = self.layout.borrow().as_ref() {
            if orientation == adw::gtk::Orientation::Vertical
                && for_size > 0
                && layout.width() != for_size * SCALE
            {
                layout.set_width(for_size * SCALE);
            }
        }

        if let Some(layout) = self.layout.borrow().as_ref() {
            let (width, height) = layout.size();
            match orientation {
                adw::gtk::Orientation::Horizontal => (0, width / SCALE),
                adw::gtk::Orientation::Vertical => (height / SCALE, height / SCALE),
                _ => (0, 0),
            }
        } else {
            (0, 0)
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for InlineView {
    const NAME: &'static str = "InlineView";
    type Type = super::InlineView;
    type ParentType = adw::gtk::Widget;
}

#[glib::derived_properties]
impl ObjectImpl for InlineView {
    fn dispose(&self) {
        // Disconnect signal when widget is being destroyed
        if let Some(signal_id) = self.buffer_signal_id.borrow_mut().take() {
            if let Some(buffer) = self.buffer.borrow().as_ref() {
                buffer.disconnect(signal_id);
            }
        }
    }
}

impl WidgetImpl for InlineView {
    fn request_mode(&self) -> adw::gtk::SizeRequestMode {
        adw::gtk::SizeRequestMode::HeightForWidth
    }

    fn measure(&self, orientation: adw::gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        let (min, nat) = self.update_layout(orientation, for_size);
        (min, nat, -1, -1)
    }

    fn snapshot(&self, snapshot: &adw::gtk::Snapshot) {
        let layout = self.layout.borrow();
        let width = self.obj().width();
        if let Some(layout) = layout.as_ref() {
            if layout.width() != width * SCALE {
                layout.set_width(width * SCALE);
            }
            snapshot.save();
            snapshot.translate(&adw::gtk::graphene::Point::new(0., 0.));
            snapshot.append_layout(layout, &self.obj().color());
            snapshot.restore();
        }
    }
}

unsafe impl<T: ObjectSubclass> IsSubclassable<T> for super::InlineView
where
    <T as ObjectSubclass>::Type: IsA<super::InlineView>,
    T: WidgetImpl,
{
    fn class_init(class: &mut adw::glib::Class<Self>) {
        Self::parent_class_init::<T>(class.upcast_ref_mut());
    }
}
