use clap::{command, Command};
use duct::cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .subcommand(Command::new("test").about("tests fete- required for doctesting"))
        .get_matches();

    if matches.subcommand_matches("test").is_some() {
        cmd!(std::env::var("CARGO")?, "test")
            .env(
                "RUSTFLAGS",
                format!(
                    "--cfg fete_doctest {}",
                    std::env::var("RUSTFLAGS").unwrap_or_else(|_| String::new())
                ),
            )
            .env(
                "RUST_BACKTRACE",
                std::env::var("RUST_BACKTRACE").unwrap_or_else(|_| String::from("0")),
            )
            .env("CARGO_TARGET_DIR", "target/xtask-test")
            .run()?;
    }

    Ok(())
}
