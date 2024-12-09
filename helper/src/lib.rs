use std::env;

pub fn pwd() {
    if let Ok(cwd) = env::current_dir() {
        log::info!("{}", cwd.display());
    }
}
