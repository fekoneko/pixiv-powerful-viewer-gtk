mod imp;

use adw::glib;

glib::wrapper! {
    pub struct PreviewPanel(ObjectSubclass<imp::PreviewPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PreviewPanel {}
