# Pixiv Powerful Viewer

> _**R**ust_ stands for _**R**ewrite_ - let's go!

The app is still very much work in progress. I'm rewriting the previous [Tauri version](https://github.com/fekoneko/pixiv-powerful-viewer) to GTK platform.

- This will allow for greatly improved performance
- Native look in Linux GNOME environmet (while still working on any other platform)
- Fewer dependencies and reduced memory consumtion

I also plan to implement my own version of batch downloader integrated into the app.

- No need to depend on the third party downloader (which is only available in Chromium)
- Custom downloader will allow for saving more useful metadata, such as series data
- Convenient YAML format
- Directory structure and filenames will remain human-friendly as they were in the previous version + no weird caches outside the collection directory
