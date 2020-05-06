extern crate grep_table_converter;

use log::*;
use pretty_env_logger;
use structopt::{clap, StructOpt};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read, Write, BufReader};
use anyhow::Context as _;
use atty::Stream;
use chrono::Utc;

use grep_table_converter::generator::*;

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

fn read_from_stdin() -> anyhow::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

fn read_from_file(filename: &String) -> anyhow::Result<String> {
    let mut buf = String::new();
    let mut f = File::open(filename)
        .with_context(|| format!("Could not open file: {}", filename))?;

    f.read_to_string(&mut buf)
     .with_context(|| format!("Something went wrong reading file: {}", filename))?;

    Ok(buf)
}

fn is_pipe() -> bool {
    ! atty::is(Stream::Stdin)
}

fn generate_filename(mode: &Mode) -> String {
    let formatted_date = Utc::now().format("%Y%m%d%H%M%S").to_string();
    String::from("output_") + formatted_date.as_str() + mode.extension()
}

fn main() -> anyhow::Result<()> {
    // initialize
    pretty_env_logger::init();
    debug!("Applicatin start");

    let opt = Opt::from_args();

    let mode = Mode::from(&opt.mode)
        .with_context(|| "Failed to read mode.")?;
    if opt.filename.is_none() && ! is_pipe() {
        Opt::clap().print_help()?;
        std::process::exit(1);
    }

    let content = match opt.filename {
        Some(fname) => read_from_file(&fname)?,
        None => read_from_stdin()?,
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
