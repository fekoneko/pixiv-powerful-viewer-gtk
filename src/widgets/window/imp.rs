use adw::glib;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::prelude::StaticTypeExt;
use gtk::CompositeTemplate;

use crate::widgets::explorer_panel::ExplorerPanel;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/fekoneko/ppv/app/application_window.ui")]
pub struct Window {}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "PpvApplicationWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        ExplorerPanel::ensure_type();

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

impl AdwApplicationWindowImpl for Window {}
