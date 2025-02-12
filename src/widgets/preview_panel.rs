use adw::glib;

mod imp {
    use adw::glib;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/fekoneko/ppv/app/preview_panel.ui")]
    pub struct PreviewPanel {}

    #[glib::object_subclass]
    impl ObjectSubclass for PreviewPanel {
        const NAME: &'static str = "PpvPreviewPanel";
        type Type = super::PreviewPanel;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(object: &InitializingObject<Self>) {
            object.init_template();
        }
    }

    impl ObjectImpl for PreviewPanel {}
    impl WidgetImpl for PreviewPanel {}
    impl BoxImpl for PreviewPanel {}

    impl PreviewPanel {}
}

glib::wrapper! {
    pub struct PreviewPanel(ObjectSubclass<imp::PreviewPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl PreviewPanel {}
