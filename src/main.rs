extern crate grep_table_converter;

use log::*;
use pretty_env_logger;
use structopt::{clap, StructOpt};
use std::fs::File;
use anyhow::Context as _;
use std::io::Write;
use atty::Stream;

use grep_table_converter::generator::*;
use grep_table_converter::io::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "grep_table_converter")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(name = "input-filename")]
    filename: Option<String>,

    #[structopt(short = "o", long = "output-filename")]
    output_filename: Option<String>,

    #[structopt(short = "mode")]
    mode: String,
}


fn main() -> anyhow::Result<()> {
    // initialize
    pretty_env_logger::init();

    let opt = Opt::from_args();

    let mode = Mode::from(&opt.mode)
        .with_context(|| "Failed to read mode.")?;
    if opt.filename.is_none() && atty::is(Stream::Stdin) {
        Opt::clap().print_help()?;
        std::process::exit(1);
    }

    let content = match opt.filename {
        Some(fname) => read_from_file(&fname)?,
        None => {
            let stdio = std::io::stdin();
            let input = stdio.lock();
            read_from_stdin(input)?
        },
    };
    if content.is_empty() {
        Opt::clap().get_matches().usage();
    }

    debug!("{}", &content);

    // generate table
    let converted = generate_table(&content, &mode)
        .with_context(|| "Failed to generate table.")?;

    debug!("{}", &converted);

    // save file
    let output_filename = match opt.output_filename {
        Some(f) => f,
        None => generate_filename(&mode),
    };
    let mut f = File::create(&output_filename)
        .with_context(|| "Failed to create file handler.")?;
    write!(f, "{}", converted)
        .with_context(|| "Failed to write converted text to file.")?;

    Ok(())
}
