
use std::fs::OpenOptions;
use std::process::exit;

#[macro_use] extern crate clap;
use clap::Arg;

mod errors;
mod generator;

use crate::errors::AppResultU;



fn main() {
    if let Err(err) = app() {
        eprintln!("{}", err);
        exit(1);
    }
}


fn app() -> AppResultU {
    let app = app_from_crate!()
        .arg(Arg::with_name("height")
             .help("Image height")
             .default_value("1000")
             .short("h")
             .long("height"))
        .arg(Arg::with_name("width")
             .help("Image width")
             .default_value("1000")
             .short("w")
             .long("width"))
        .arg(Arg::with_name("length")
             .help("Length of fern")
             .default_value("10")
             .short("l")
             .long("length"))
        .arg(Arg::with_name("frames")
             .help("Number of animation frames")
             .default_value("4")
             .short("f"))
        .arg(Arg::with_name("interval")
             .help("Animation interval (msec)")
             .default_value("500")
             .short("i"))
        .arg(Arg::with_name("color")
             .help("Fern color (CSS Color)")
             .default_value("rgba (0, 255, 0, 1.0)")
             .long("color")
             .short("c"))
        .arg(Arg::with_name("background")
             .help("Fern color (CSS Color)")
             .default_value("rgba (255, 255, 255, 0.0)")
             .long("background")
             .short("b"))
        .arg(Arg::with_name("filepath")
             .help("Output file")
             .required(true))
        ;

    let matches = app.get_matches();

    let background_color: &str = matches.value_of("background").unwrap();
    let fern_color: &str = matches.value_of("color").unwrap();
    let filepath: &str = matches.value_of("filepath").unwrap();
    let frames: u32 = matches.value_of("frames").unwrap().parse()?;
    let height: u32 = matches.value_of("height").unwrap().parse()?;
    let interval: u16 = matches.value_of("interval").unwrap().parse()?;
    let length: u32 = matches.value_of("length").unwrap().parse()?;
    let width: u32 = matches.value_of("width").unwrap().parse()?;

    let config = generator::Config {
        background_color: background_color.parse()?,
        fern_color: fern_color.parse()?,
        frames,
        height,
        interval,
        length,
        width,
    };

    let mut file = OpenOptions::new().write(true).create(true).open(filepath)?;

    let mut gen = generator::Generator::new(config);
    gen.generate(&mut file)?;

    Ok(())
}
