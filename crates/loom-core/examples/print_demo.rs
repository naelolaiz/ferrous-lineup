fn main() {
    print!(
        "{}",
        loom_core::file::to_text(&loom_core::Project::demo()).unwrap()
    );
}
