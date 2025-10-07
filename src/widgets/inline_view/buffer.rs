use std::cell::RefCell;

use adw::gdk;
use adw::gtk::glib;
use adw::gtk::pango;
use adw::gtk::subclass::prelude::*;
use glib::prelude::*;
use glib_macros::Properties;

/// Opaque handle to a position in the text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, glib::Boxed)]
#[boxed_type(name = "InlinePos")]
pub struct InlinePos {
    offset: usize,
}

impl InlinePos {
    pub(crate) fn new(offset: usize) -> Self {
        Self { offset }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
/// Opaque handle to an anchor in the buffer (for inserting images)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, glib::Boxed)]
#[boxed_type(name = "InlineAnchor")]
pub struct InlineAnchor {
    id: usize,
}

impl InlineAnchor {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// Text attribute for styling
#[derive(Debug, Clone)]
pub enum TextAttr {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Color(adw::gtk::gdk::RGBA),
    Link(String),
    FontSize(i32),
    FontFamily(String),
}

#[derive(Debug, Clone, Copy, glib::Enum)]
#[enum_type(name = "TextAttrType")]
pub enum TextAttrType {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Color,
    Link,
    FontSize,
    FontFamily,
}

impl TextAttr {
    /// Get the type of this attribute
    pub fn attr_type(&self) -> TextAttrType {
        match self {
            TextAttr::Bold => TextAttrType::Bold,
            TextAttr::Italic => TextAttrType::Italic,
            TextAttr::Underline => TextAttrType::Underline,
            TextAttr::Strikethrough => TextAttrType::Strikethrough,
            TextAttr::Color(_) => TextAttrType::Color,
            TextAttr::Link(_) => TextAttrType::Link,
            TextAttr::FontSize(_) => TextAttrType::FontSize,
            TextAttr::FontFamily(_) => TextAttrType::FontFamily,
        }
    }

    /// Apply this attribute to a Pango attribute list
    pub(crate) fn apply_to_pango(&self, attr_list: &pango::AttrList, start: u32, end: u32) {
        match self {
            TextAttr::Bold => {
                let mut attr = pango::AttrInt::new_weight(pango::Weight::Bold);
                attr.set_start_index(start);
                attr.set_end_index(end);
                attr_list.insert(attr);
            }
            TextAttr::Italic => {
                let mut attr = pango::AttrInt::new_style(pango::Style::Italic);
                attr.set_start_index(start);
                attr.set_end_index(end);
                attr_list.insert(attr);
            }
            TextAttr::Underline => {
                let mut attr = pango::AttrInt::new_underline(pango::Underline::Single);
                attr.set_start_index(start);
                attr.set_end_index(end);
                attr_list.insert(attr);
            }
            TextAttr::Strikethrough => {
                let mut attr = pango::AttrInt::new_strikethrough(true);
                attr.set_start_index(start);
                attr.set_end_index(end);
                attr_list.insert(attr);
            }
            TextAttr::Color(color) => {
                let mut attr = pango::AttrColor::new_foreground(
                    (color.red() * 65535.0) as u16,
                    (color.green() * 65535.0) as u16,
                    (color.blue() * 65535.0) as u16,
                );
                attr.set_start_index(start);
                attr.set_end_index(end);
                attr_list.insert(attr);
            }
            TextAttr::Link(_url) => {
                // For links, we use blue color and underline
                let mut color_attr = pango::AttrColor::new_foreground(0, 0, 65535);
                color_attr.set_start_index(start);
                color_attr.set_end_index(end);
                attr_list.insert(color_attr);

                let mut underline_attr = pango::AttrInt::new_underline(pango::Underline::Single);
                underline_attr.set_start_index(start);
                underline_attr.set_end_index(end);
                attr_list.insert(underline_attr);
            }
            TextAttr::FontSize(size) => {
                let mut attr = pango::AttrSize::new(*size * pango::SCALE);
                attr.set_start_index(start);
                attr.set_end_index(end);
                attr_list.insert(attr);
            }
            TextAttr::FontFamily(family) => {
                let mut attr = pango::AttrString::new_family(family);
                attr.set_start_index(start);
                attr.set_end_index(end);
                attr_list.insert(attr);
            }
        }
    }
}

/// Stored attribute with its range
#[derive(Debug, Clone)]
struct AttributeSpan {
    attr: TextAttr,
    start: InlinePos,
    end: InlinePos,
}

/// Anchor data
#[derive(Debug, Clone)]
struct AnchorData {
    id: usize,
    pos: InlinePos,
    paintable: Option<gdk::Paintable>,
}

mod imp {
    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::InlineBuffer)]
    pub struct InlineBuffer {
        #[property(get, set = Self::set_text)]
        pub(super) text: RefCell<String>,
        pub(super) attributes: RefCell<Vec<AttributeSpan>>,
        pub(super) anchors: RefCell<Vec<AnchorData>>,
        pub(super) next_anchor_id: RefCell<usize>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for InlineBuffer {
        const NAME: &'static str = "InlineBuffer";
        type Type = super::InlineBuffer;
    }

    #[glib::derived_properties]
    impl ObjectImpl for InlineBuffer {
        fn signals() -> &'static [glib::subclass::Signal] {
            use std::sync::OnceLock;
            static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    // Signal emitted when the buffer content changes
                    glib::subclass::Signal::builder("changed").build(),
                    // Signal emitted when an attribute is added: (attr_type: TextAttrType, start: InlinePos, end: InlinePos)
                    glib::subclass::Signal::builder("attribute-added")
                        .param_types([
                            TextAttrType::static_type(),
                            InlinePos::static_type(),
                            InlinePos::static_type(),
                        ])
                        .build(),
                    // Signal emitted when an anchor is created: (anchor: InlineAnchor, position: InlinePos)
                    glib::subclass::Signal::builder("anchor-created")
                        .param_types([InlineAnchor::static_type(), InlinePos::static_type()])
                        .build(),
                    // Signal emitted when a paintable is inserted at an anchor: (anchor: InlineAnchor)
                    glib::subclass::Signal::builder("paintable-inserted")
                        .param_types([InlineAnchor::static_type()])
                        .build(),
                ]
            })
        }
    }

    impl InlineBuffer {
        /// Set the text content (clears all attributes and anchors)
        pub fn set_text(&self, text: String) {
            self.text.replace(text);
            self.attributes.borrow_mut().clear();
            self.anchors.borrow_mut().clear();

            // Emit changed signal
            self.obj().emit_by_name::<()>("changed", &[]);
        }

        /// Check if the buffer is empty
        pub fn is_empty(&self) -> bool {
            self.text.borrow().is_empty()
        }

        /// Append text to the buffer and return the position at the end
        pub fn push_str(&self, text: &str) -> InlinePos {
            let mut buffer = self.text.borrow_mut();
            buffer.push_str(text);
            let pos = InlinePos::new(buffer.len());
            drop(buffer);

            // Emit changed signal
            self.obj().emit_by_name::<()>("changed", &[]);
            pos
        }

        /// Apply an attribute to a range of text
        pub fn apply_attribute(&self, start: InlinePos, end: InlinePos, attr: TextAttr) {
            let attr_type = attr.attr_type();
            let span = AttributeSpan { attr, start, end };
            self.attributes.borrow_mut().push(span);

            // Emit signals
            self.obj()
                .emit_by_name::<()>("attribute-added", &[&attr_type, &start, &end]);
            self.obj().emit_by_name::<()>("changed", &[]);
        }

        /// Create an anchor at a specific position
        pub fn push_anchor(&self) -> InlineAnchor {
            let mut anchors = self.anchors.borrow_mut();
            let id = self.next_id();
            let pos = self.push_str("\u{FFFC}"); // Object Replacement Character to represent the anchor in text

            let anchor_data = AnchorData {
                id,
                pos,
                paintable: None,
            };
            anchors.push(anchor_data);

            let anchor = InlineAnchor::new(id);

            // Emit signal
            self.obj()
                .emit_by_name::<()>("anchor-created", &[&anchor, &pos]);

            anchor
        }

        /// Insert a paintable (e.g., image) at an anchor
        pub fn insert_paintable_at_anchor(&self, anchor: InlineAnchor, paintable: &gdk::Paintable) {
            let mut anchors = self.anchors.borrow_mut();
            if let Some(anchor_data) = anchors.iter_mut().find(|a| a.id == anchor.id()) {
                anchor_data.paintable = Some(paintable.clone());

                // Emit signals
                drop(anchors); // Release borrow before emit
                self.obj()
                    .emit_by_name::<()>("paintable-inserted", &[&anchor]);
                self.obj().emit_by_name::<()>("changed", &[]);
            }
        }

        /// Get the position of an anchor
        pub fn get_anchor_position(&self, anchor: InlineAnchor) -> Option<InlinePos> {
            self.anchors
                .borrow()
                .iter()
                .find(|a| a.id == anchor.id())
                .map(|a| a.pos)
        }

        /// Get the start position of the buffer
        pub fn start_pos(&self) -> InlinePos {
            InlinePos::new(0)
        }

        /// Get the end position of the buffer
        pub fn current_pos(&self) -> InlinePos {
            InlinePos::new(self.text.borrow().len())
        }

        /// Clear all content, attributes, and anchors
        pub fn clear(&self) {
            self.text.borrow_mut().clear();
            self.attributes.borrow_mut().clear();
            self.anchors.borrow_mut().clear();
        }

        /// Build a Pango attribute list from the stored attributes
        pub fn build_pango_attributes(&self) -> pango::AttrList {
            let attr_list = pango::AttrList::new();
            let attributes = self.attributes.borrow();

            for span in attributes.iter() {
                let start = span.start.offset() as u32;
                let end = span.end.offset() as u32;
                span.attr.apply_to_pango(&attr_list, start, end);
            }

            attr_list
        }

        fn next_id(&self) -> usize {
            self.next_anchor_id.replace_with(|&mut id| id + 1)
        }
    }
}

glib::wrapper! {
    pub struct InlineBuffer(ObjectSubclass<imp::InlineBuffer>);
}

impl InlineBuffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.imp().is_empty()
    }

    /// Append text to the buffer and return the position at the end
    pub fn push_str(&self, text: &str) -> InlinePos {
        self.imp().push_str(text)
    }

    /// Apply an attribute to a range of text
    pub fn apply_attribute(&self, start: InlinePos, end: InlinePos, attr: TextAttr) {
        self.imp().apply_attribute(start, end, attr)
    }

    /// Create an anchor at a specific position
    pub fn push_anchor(&self) -> InlineAnchor {
        self.imp().push_anchor()
    }

    /// Insert a paintable (e.g., image) at an anchor
    pub fn insert_paintable_at_anchor(&self, anchor: InlineAnchor, paintable: &gdk::Paintable) {
        self.imp().insert_paintable_at_anchor(anchor, paintable)
    }

    /// Get the position of an anchor
    pub fn get_anchor_position(&self, anchor: InlineAnchor) -> Option<InlinePos> {
        self.imp().get_anchor_position(anchor)
    }

    /// Get the start position of the buffer
    pub fn start_pos(&self) -> InlinePos {
        self.imp().start_pos()
    }

    /// Get the end position of the buffer
    pub fn current_pos(&self) -> InlinePos {
        self.imp().current_pos()
    }

    /// Clear all content, attributes, and anchors
    pub fn clear(&self) {
        self.imp().clear()
    }

    /// Build a Pango attribute list from the stored attributes
    pub(crate) fn build_pango_attributes(&self) -> pango::AttrList {
        self.imp().build_pango_attributes()
    }
}

impl Default for InlineBuffer {
    fn default() -> Self {
        Self::new()
    }
}
