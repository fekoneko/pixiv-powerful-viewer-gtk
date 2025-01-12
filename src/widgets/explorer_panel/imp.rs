use adw::glib;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::CompositeTemplate;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/fekoneko/ppv/app/explorer_panel.ui")]
pub struct ExplorerPanel {}

#[glib::object_subclass]
impl ObjectSubclass for ExplorerPanel {
    const NAME: &'static str = "PpvExplorerPanel";
    type Type = super::ExplorerPanel;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(object: &InitializingObject<Self>) {
        object.init_template();
    }
}

impl ObjectImpl for ExplorerPanel {}

impl WidgetImpl for ExplorerPanel {}

impl BoxImpl for ExplorerPanel {}
