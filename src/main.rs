mod app;
mod verrors;

fn main() {
    if let Err(e) = app::run(
        // std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        // false,
    ) {
        eprintln!("{}", e);
    }
}
