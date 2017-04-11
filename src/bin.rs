extern crate clap;
extern crate towerslib;

use clap::{App, Arg};
use towerslib::*;

fn main() {
    let matches = App::new("Towers of Hanoi")
        .version("1.0.0")
        .author("Christian Douven")
        .about("A Rust introductionary tutorial")
        .arg(Arg::with_name("num_disks")
            .short("d")
            .long("disks")
            .value_name("DISKS")
            .required(true)
            .help("Sets the number of disks")
            .takes_value(true))
        .arg(Arg::with_name("take")
            .short("t")
            .long("take")
            .value_name("TAKE")
            .help("The number of results to display")
            .takes_value(true))
        .arg(Arg::with_name("skip")
            .short("s")
            .long("skip")
            .value_name("SKIP")
            .help("The number of first results to skip")
            .takes_value(true))
        .get_matches();

    let num_disks: usize = matches.value_of("num_disks").unwrap().parse().unwrap();
    let skip: usize = matches.value_of("skip").unwrap_or("0").parse().unwrap();
    let take: usize =
        matches.value_of("take").map(|s| s.parse().unwrap()).unwrap_or(std::usize::MAX);

    let player = StupidPlayer::new("Joe");
    let mut session = Session::with_initial_disks(player, num_disks);

    for (i, action) in session.iter().enumerate().skip(skip).take(take) {
        println!("{}: {:?}", i, action);
    }
}
