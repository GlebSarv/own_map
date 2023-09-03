mod directory_reader;
mod message;
use clap::{App, Arg};

fn main() {
    let matches = App::new("exif_reader")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .arg(
            Arg::with_name("directory")
                .short('d')
                .long("directory")
                .help("Specify directory with photo")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let directory = matches.value_of("directory").unwrap();
    directory_reader::walking(directory);
}