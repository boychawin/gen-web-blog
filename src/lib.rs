pub mod app;
mod blog;
pub mod deploy;
pub use shared::error as error;
pub mod generator;
mod posts;
pub mod shared;
pub use shared::constants as constants;
pub use shared::language as language;
pub use shared::template_manager as template_manager;
pub use shared::tailwind as tailwind;
pub use shared::validation as validation;

#[cfg(test)]
mod tests;
use eyre::Result;
use generator::Generator;
use log::error;

pub fn main() -> Result<()> {
    let _ = env_logger::try_init();

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    
    println!("\x1b[96m┌{:-^48}┐\x1b[0m", "");
    println!("\x1b[96m│\x1b[97m  {} v{}{:>28}│\x1b[0m", name, version, "");
    println!("\x1b[96m└{:-^48}┘\x1b[0m", "");
    let blog = match Generator::new("build", "contents") {
        Ok(generator) => generator,
        Err(e) => {
            error!("│  🔥 Failed to initialize blog generator: {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = blog.generate() {
        error!("│  🔥 Failed to generate blog: {e}");
        std::process::exit(1);
    }

    Ok(())
}