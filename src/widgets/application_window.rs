use adw::{gio, glib, Application};
use glib::Object;

mod imp {
    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;

    use crate::widgets::collection_view::CollectionView;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/fekoneko/ppv/app/application_window.ui")]
    pub struct ApplicationWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for ApplicationWindow {
        const NAME: &'static str = "PpvApplicationWindow";
        type Type = super::ApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            CollectionView::ensure_type();
            klass.bind_template();
        }

        fn instance_init(object: &InitializingObject<Self>) {
            object.init_template();
        }
    }

    impl ObjectImpl for ApplicationWindow {}
    impl WidgetImpl for ApplicationWindow {}
    impl WindowImpl for ApplicationWindow {}
    impl ApplicationWindowImpl for ApplicationWindow {}
    impl AdwApplicationWindowImpl for ApplicationWindow {}
}

glib::wrapper! {
    pub struct ApplicationWindow(ObjectSubclass<imp::ApplicationWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ApplicationWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }
}
