use log::*;
use pretty_env_logger;
use structopt::{clap, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(name = "grepross")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(name = "filename")]
    filename: String,
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    debug!("opt: {:?}", opt)
}
