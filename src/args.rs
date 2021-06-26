use std::net::SocketAddr;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp], after_help=r#"The original shijack in C was written by spwny and released around 2001.
shijack credited cyclozine for inspiration."#)]
pub struct Args {
    
    pub interface: String,
    pub src: SocketAddr,
    pub dst: SocketAddr,

    #[structopt(long)]
    pub seq: Option<u32>,

    #[structopt(long)]
    pub ack: Option<u32>,

    #[structopt(short = "r", long)]
    pub reset: bool,

    #[structopt(short = "0", long)]
    pub send_null: bool,

    #[structopt(short, long, parse(from_occurrences))]
    pub quiet: u8,
}
