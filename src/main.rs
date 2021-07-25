extern crate clap;
use clap::{Arg, App, value_t};
use std::{error::Error, boxed::Box};
use nix::unistd::Pid;

mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("pid")
            .short("p")
            .long("pid")
            .value_name("PID")
            .help("The pid to wait for")
            .takes_value(true)
            .required(true)
            .conflicts_with("search"))
        .arg(Arg::with_name("search")
            .short("s")
            .long("search")
            .value_name("SEARCH_TERM")
            .help("Search term for target process")
            .takes_value(true)
            .required(true)
            .conflicts_with("pid"))
        .get_matches();

    let pid = if matches.is_present("pid") {
        Pid::from_raw(value_t!(matches, "pid", i32).unwrap_or_else(|e| e.exit()))
    } else {
        lib::get_pid_for_cmd(matches.value_of("search").unwrap())?
    };
    
    lib::wait_for_pid(pid)
}
