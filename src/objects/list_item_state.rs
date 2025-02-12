use adw::glib;

mod imp {
    use adw::glib;
    use adw::glib::Properties;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use std::cell::Cell;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ListItemState)]
    pub struct ListItemState {
        #[property(get, set)]
        index: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ListItemState {
        const NAME: &'static str = "PpvListItemState";
        type Type = super::ListItemState;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ListItemState {}
}

glib::wrapper! {
    pub struct ListItemState(ObjectSubclass<imp::ListItemState>);
}

impl ListItemState {
    pub fn new(index: i32) -> Self {
        glib::Object::builder().property("index", index).build()
    }
}
