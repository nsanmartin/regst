use clap::{App, Arg};

pub fn get_matches() {

    App::new("reg")
        .arg(Arg::new("print").short('p').long("print"))
        .get_matches();
}
