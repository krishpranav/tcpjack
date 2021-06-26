use env_logger::Env;
use rshijack::args::Args;
use rshijack::errors::*;
use rshijack::net::{self, TcpFlags};
use std::io::{self, Read};
use std::thread;
use structopt::StructOpt;