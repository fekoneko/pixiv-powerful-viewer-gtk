use adw::glib;

mod imp {
    use adw::gio;
    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;
    use gtk::SignalListItemFactory;

    use crate::objects::ListItemState;
    use crate::widgets::WorkCard;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/fekoneko/ppv/app/explorer_panel.ui")]
    pub struct ExplorerPanel {
        #[template_child]
        works_list_scrolled_window: TemplateChild<gtk::ScrolledWindow>,
    }

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

    impl ObjectImpl for ExplorerPanel {
        fn constructed(&self) {
            let item_states: Vec<_> = (0..1_000_000).map(|i| ListItemState::new(i)).collect();
            let model = gio::ListStore::new::<ListItemState>();
            model.extend_from_slice(&item_states);

            let item_factory = SignalListItemFactory::new();

            item_factory.connect_setup(move |_, item| {
                let work_card = WorkCard::new();
                item.downcast_ref::<gtk::ListItem>()
                    .unwrap()
                    .set_child(Some(&work_card));
            });

            item_factory.connect_bind(move |_, list_item| {
                let item_state = list_item
                    .downcast_ref::<gtk::ListItem>()
                    .unwrap()
                    .item()
                    .and_downcast::<ListItemState>()
                    .unwrap();

                let work_card = list_item
                    .downcast_ref::<gtk::ListItem>()
                    .unwrap()
                    .child()
                    .and_downcast::<WorkCard>()
                    .unwrap();

                work_card.display_work_with_index(item_state.index());
            });

            let selection_model = gtk::SingleSelection::new(Some(model));
            let works_list = gtk::ListView::new(Some(selection_model), Some(item_factory));
            self.works_list_scrolled_window.set_child(Some(&works_list));
        }
    }

    impl WidgetImpl for ExplorerPanel {}
    impl BoxImpl for ExplorerPanel {}

    impl ExplorerPanel {}
}

glib::wrapper! {
    pub struct ExplorerPanel(ObjectSubclass<imp::ExplorerPanel>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ExplorerPanel {}
