//! One binary, twenty tiny “apps”. Build with `cargo run -- <sub‑command>`.

use clap::{Parser, Subcommand};

mod x1;
mod x2;
mod x3;
mod x4;
mod x5;
mod x6;
mod x7;
mod x8;
mod x9;
mod x10;
mod x11;
mod x12;
mod x13;
mod x14;
mod x15;
mod x16;
mod x17;
mod x18;
mod x19;
mod x20;

/// Top‑level CLI definition.
#[derive(Parser)]
#[command(
    name    = "toolbox",
    about   = "A single executable wrapping 20 little utilities.",
    version = env!("CARGO_PKG_VERSION"),
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// One sub‑command per module.
/// `alias = "1"` lets users type either `x1` *or* just `1`.
#[derive(Subcommand)]
enum Command {
    #[command(alias = "1")]
    X1,
    #[command(alias = "2")]
    X2,
    #[command(alias = "3")]
    X3,
    #[command(alias = "4")]
    X4,
    #[command(alias = "5")]
    X5,
    #[command(alias = "6")]
    X6,
    #[command(alias = "7")]
    X7,
    #[command(alias = "8")]
    X8,
    #[command(alias = "9")]
    X9,
    #[command(alias = "10")]
    X10,
    #[command(alias = "11")]
    X11,
    #[command(alias = "12")]
    X12,
    #[command(alias = "13")]
    X13,
    #[command(alias = "14")]
    X14,
    #[command(alias = "15")]
    X15,
    #[command(alias = "16")]
    X16,
    #[command(alias = "17")]
    X17,
    #[command(alias = "18")]
    X18,
    #[command(alias = "19")]
    X19,
    #[command(alias = "20")]
    X20,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::X1  => x1::run(),
        Command::X2  => x2::run(),
        Command::X3  => x3::run(),
        Command::X4  => x4::run(),
        Command::X5  => x5::run(),
        Command::X6  => x6::run(),
        Command::X7  => x7::run(),
        Command::X8  => x8::run(),
        Command::X9  => x9::run(),
        Command::X10 => x10::run(),
        Command::X11 => x11::run(),
        Command::X12 => x12::run(),
        Command::X13 => x13::run(),
        Command::X14 => x14::run(),
        Command::X15 => x15::run(),
        Command::X16 => x16::run(),
        Command::X17 => x17::run(),
        Command::X18 => x18::run(),
        Command::X19 => x19::run(),
        Command::X20 => x20::run(),
    }
}
