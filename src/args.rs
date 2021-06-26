use std::net::SocketAddr;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp], after_help=r#"The original tcpshark in C was written by spwny and released around 2001.
tcpshark credited cyclozine for inspiration."#)]
