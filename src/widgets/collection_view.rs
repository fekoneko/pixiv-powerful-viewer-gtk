use adw::glib;

mod imp {
    use adw::glib;
    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;
    use gtk::FileDialog;

    use crate::library::collection_reader::CollectionReader;
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
            self.open_collection_button.set_sensitive(false);

            let file_dialog = FileDialog::builder().title("Open Collection").build();
            if let Ok(Some(path)) = file_dialog
                .select_folder_future(None::<&gtk::Window>)
                .await
                .map(|dir| dir.path())
            {
                let (mut collection_reader, errors) = CollectionReader::new(path).await;
                println!("Parsed collection structure with {} errors", errors.len());
                while let Some(work) = collection_reader.next_work().await {
                    match work {
                        Ok(work) => {
                            println!(
                                "Loaded work {}",
                                work.metadata.title.unwrap_or(String::from("Unknown"))
                            )
                        }
                        Err(error) => println!("Failed to load work: {:?}", error),
                    }
                }
            }
            self.open_collection_button.set_sensitive(true);
        }
    }
}

glib::wrapper! {
    pub struct CollectionView(ObjectSubclass<imp::CollectionView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl CollectionView {}
