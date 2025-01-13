mod imp;

use adw::glib;

glib::wrapper! {
    pub struct CollectionView(ObjectSubclass<imp::CollectionView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CollectionView {}
