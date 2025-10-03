fn main() {
    glib_build_tools::compile_resources(
        &["data"],
        "data/patato-md.gresource.xml",
        "patato-md.gresource",
    );
}
