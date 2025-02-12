use adw::glib;

mod imp {
    use adw::glib;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/fekoneko/ppv/app/work_card.ui")]
    pub struct WorkCard {}

    #[glib::object_subclass]
    impl ObjectSubclass for WorkCard {
        const NAME: &'static str = "PpvWorkCard";
        type Type = super::WorkCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(object: &InitializingObject<Self>) {
            object.init_template();
        }
    }

    impl ObjectImpl for WorkCard {}
    impl WidgetImpl for WorkCard {}
    impl BoxImpl for WorkCard {}

    impl WorkCard {}
}

glib::wrapper! {
    pub struct WorkCard(ObjectSubclass<imp::WorkCard>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WorkCard {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn display_work_with_index(&self, index: i32) {
        println!("display_work_with_index({})", index)
    }
}
