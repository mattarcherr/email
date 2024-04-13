mod app;
mod verrors;
mod context;
mod control;

fn main() {
    if let Err(e) = app::launch(
        // std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        // false,
    ) {
        eprintln!("{}", e);
    }
}
