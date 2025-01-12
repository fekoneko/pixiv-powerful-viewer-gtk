mod imp;

use adw::glib;

glib::wrapper! {
    pub struct ExplorerPanel(ObjectSubclass<imp::ExplorerPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ExplorerPanel {}
