fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/ppv.gresource.xml",
        "ppv.gresource",
    );
}
