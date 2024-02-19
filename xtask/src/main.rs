use clap::{arg, command, Command};
use duct::cmd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .subcommand(
            Command::new("test")
                .about("tests fete- required for doctesting")
                .arg(
                    arg!(<args> ... "extra arguments to pass directly to `cargo test`").last(true),
                ),
        )
        .get_matches();

    if let Some(sub) = matches.subcommand_matches("test") {
        let extra: Vec<&str> = sub
            .get_many::<String>("args")
            .map(|m| m.map(|v| &**v).collect())
            .unwrap_or_else(Vec::new);

        let mut args = Vec::with_capacity(extra.len() + 1);
        args.insert(0, "test");
        args.extend_from_slice(&extra);

        cmd(std::env::var("CARGO")?, args)
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
