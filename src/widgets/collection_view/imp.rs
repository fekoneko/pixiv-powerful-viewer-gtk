use adw::glib;
use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::CompositeTemplate;

use crate::library::collection::Collection;
use crate::widgets::explorer_panel::ExplorerPanel;
use crate::widgets::preview_panel::PreviewPanel;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/fekoneko/ppv/app/collection_view.ui")]
pub struct CollectionView {
    #[template_child]
    open_collection_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionView {
    const NAME: &'static str = "PpvCollectionView";
    type Type = super::CollectionView;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        ExplorerPanel::ensure_type();
        PreviewPanel::ensure_type();
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(object: &InitializingObject<Self>) {
        object.init_template();
    }
}

impl ObjectImpl for CollectionView {}
impl WidgetImpl for CollectionView {}
impl BoxImpl for CollectionView {}

#[gtk::template_callbacks]
impl CollectionView {
    #[template_callback]
    async fn handle_open_collection(&self) {
        let collection = Collection::new(String::from("/"));

        self.open_collection_button.set_sensitive(false);
        if let Ok((works, errors)) = collection.works().await {}
        self.open_collection_button.set_sensitive(true);
    }
}
