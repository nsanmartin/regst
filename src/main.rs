extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::env;
use std::io::{self, BufRead};
use clap::Parser;
//use std::path;
//use dirs::home_dir;

static REGFILE_SIZE: u32 = 39;

fn get_reg_filename() -> String {
    let mut path = dirs::home_dir().unwrap();
    path.push(".reg");
    path.push("regfile");
    path.into_os_string().into_string().unwrap()
}

fn stdin_to_lines() -> Vec<String> {
    //todo: sacar el lock
    let stdin = io::stdin();
    return stdin.lock().lines().collect::<Result<_, _>>().unwrap();
}

fn store_stdin() {
    let reg_fname = get_reg_filename();
    let mut reglines = read_lines(&reg_fname);
    let mut stdin = stdin_to_lines();
    reglines.append(&mut stdin);
    std::fs::write(&reg_fname, reglines.join("\n")).expect("could not write regfile");
}

fn read_lines(fname: &str) -> Vec<String> {
    let file = std::fs::File::open(fname).expect("Error opening file");
    let reader = std::io::BufReader::new(&file);
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>().unwrap();
    return lines;
}

#[derive(Parser, Debug)]
#[clap(name = "opts")]
struct Opts {
    /// Name of the person to greet
    #[clap(short, long)]
    na: String,

    /// Number of times to greet
    #[clap(short, long, default_value = "1")]
    count: u8,

    #[clap(short, long)]
    print: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", get_reg_filename());
    if args.len() > 1 {
        println!("{:?}", args);
        let opts = Opts::parse();

        match opts.print {
            true => println!("print"),
            _ => {}
        }

        for _ in 0..opts.count {
            println!("Opts {}!", opts.na)
        }

    } else {
        store_stdin();
    }
}
