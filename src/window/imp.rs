use crate::widgets::{InlineBuffer, InlineView, TextAttr};
use adw::{glib, glib::subclass::InitializingObject, gtk, subclass::prelude::*};

#[derive(Default, gtk4_macros::CompositeTemplate)]
#[template(resource = "/com/example/potato-md/ui/window.ui")]
pub struct PotatoWindow {
    #[template_child]
    title_inline_view: TemplateChild<InlineView>,
    #[template_child]
    content_inline_view: TemplateChild<InlineView>,
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

impl ObjectImpl for PotatoWindow {
    fn constructed(&self) {
        self.parent_constructed();

        let title_buffer = InlineBuffer::new();
        let content_buffer = InlineBuffer::new();

        self.title_inline_view.set_buffer(Some(&title_buffer));
        self.content_inline_view.set_buffer(Some(&content_buffer));

        glib::MainContext::default().spawn_local(glib_macros::clone!(
            #[weak]
            title_buffer,
            #[weak]
            content_buffer,
            async move {
                glib::timeout_future_seconds(1).await;
                title_buffer.set_text("Welcome to Potato MD!");
                glib::timeout_future_seconds(1).await;
                content_buffer.set_text("This is the content area.");
            }
        ));
    }
}

impl WidgetImpl for PotatoWindow {}

impl WindowImpl for PotatoWindow {}

impl ApplicationWindowImpl for PotatoWindow {}

impl AdwApplicationWindowImpl for PotatoWindow {}
