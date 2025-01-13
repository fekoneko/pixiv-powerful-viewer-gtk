use adw::glib;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::CompositeTemplate;

use crate::widgets::explorer_panel::ExplorerPanel;
use crate::widgets::preview_panel::PreviewPanel;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/fekoneko/ppv/app/collection_view.ui")]
pub struct CollectionView {}

#[glib::object_subclass]
impl ObjectSubclass for CollectionView {
    const NAME: &'static str = "PpvCollectionView";
    type Type = super::CollectionView;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        ExplorerPanel::ensure_type();
        PreviewPanel::ensure_type();
        klass.bind_template();
    }

    fn instance_init(object: &InitializingObject<Self>) {
        object.init_template();
    }
}

impl ObjectImpl for CollectionView {}

impl WidgetImpl for CollectionView {}

impl BoxImpl for CollectionView {}
