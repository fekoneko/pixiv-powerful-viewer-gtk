use glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/fekoneko/ppv/app/window.ui")]
pub struct Window {}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "PpvApplicationWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(object: &InitializingObject<Self>) {
        object.init_template();
    }
}

impl ObjectImpl for Window {}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}
