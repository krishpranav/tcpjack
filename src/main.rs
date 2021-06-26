use env_logger::Env;
use rshijack::args::Args;
use rshijack::errors::*;
use rshijack::net::{self, TcpFlags};
use std::io::{self, Read};
use std::thread;
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = Args::from_args();

    let log_level = if args.quiet == 0 {
        "rshijack=debug"
    } else {
        "warn"
    };
}