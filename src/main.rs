mod gui;

fn main() {
    if let Err(err) = gui::run() {
        eprintln!("Gemsmith failed to start: {err:?}");
        std::process::exit(1);
    }
}
