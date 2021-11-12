extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::env;
use clap::{App, Arg};

mod store;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let matches = App::new("reg")
            .arg(Arg::new("print").short('p').long("print"))
            .arg(Arg::new("reg"))
            .get_matches();
        
        if matches.is_present("print") {
            store::get_regs().iter().rev().for_each(|r| println!("{}",r));
        }

        if let Some(reg) = matches.value_of("reg") {
            //TODO: process reg
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            let the_string = "Hello, world2!";
            ctx.set_contents(the_string.to_owned()).unwrap();
        }
        //let args: Vec<String> = env::args().collect();
    } else {
        if let Some(line) = store::append_stdin() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(line.to_owned()).unwrap();
        }
    }
}
