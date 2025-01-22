use std::io;
use std::path::PathBuf;
use std::time::Instant;

use adw::gio;

use super::work::Work;

pub struct Collection {
    path: PathBuf,
}

impl Collection {
    pub fn new(path: String) -> Self {
        let path = PathBuf::from(path);

        Self { path }
    }

    pub async fn works(&self) -> io::Result<(Vec<Work>, Vec<io::Error>)> {
        let path = self.path.clone();

        fn parse_work(path: &PathBuf) -> io::Result<Work> {
            Ok(Work {
                path: path.to_string_lossy().to_string(),
                title: path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            })
        }

        fn read_dir(
            path: &PathBuf,
            errors: &mut Vec<io::Error>,
            parse_join_handles: &mut Vec<gio::JoinHandle<io::Result<Work>>>,
        ) {
            let entries = path.read_dir();
            let Ok(entries) = entries else {
                errors.push(entries.unwrap_err());
                return;
            };

            for entry in entries {
                let Ok(entry) = entry else {
                    errors.push(entry.unwrap_err());
                    continue;
                };
                let file_type = entry.file_type();
                let Ok(file_type) = file_type else {
                    errors.push(file_type.unwrap_err());
                    continue;
                };

                if file_type.is_dir() {
                    read_dir(&entry.path(), errors, parse_join_handles);
                } else if file_type.is_file() {
                    // gio::spawn_blocking itself is very expensive
                    parse_join_handles.push(gio::spawn_blocking(move || parse_work(&entry.path())));
                }
            }
        }

        let join_handle = gio::spawn_blocking(move || {
            let mut parse_join_handles = Vec::new();
            let mut errors = Vec::new();

            let start_time = Instant::now();
            println!("Started read_dir on a thread");
            read_dir(&path, &mut errors, &mut parse_join_handles);
            println!("Finished read_dir in {:?}", start_time.elapsed());

            (parse_join_handles, errors)
        });

        println!("Reading the collection...");
        let (parse_join_handles, mut errors) = join_handle.await.unwrap();
        let mut works = Vec::new();
        println!("Recieved parse_join_handles and errors on the main thread");

        let start_time = Instant::now();
        println!("Awaiting parse_join_handles...");
        for parse_join_handle in parse_join_handles {
            match parse_join_handle.await.unwrap() {
                Ok(work) => works.push(work),
                Err(error) => errors.push(error),
            }
        }
        println!("Finished awaiting in {:?}", start_time.elapsed());

        Ok((works, errors))
    }
}
