extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::env;
use clap::{App, Arg};
use std::process::Command;

mod store;


// There are ten types of registers:		*registers* *{register}* *E354*
// 1. The unnamed register ""
// 2. 10 numbered registers "0 to "9
// 3. The small delete register "-
// 4. 26 named registers "a to "z or "A to "Z
// 5. Three read-only registers ":, "., "%
// 6. Alternate buffer register "#
// 7. The expression register "=
// 8. The selection and drop registers "*, "+ and "~ 
// 9. The black hole register "_
// 10. Last search pattern register "/

static REGS: &str= "0123456789abcdefghijklmnopqrstuvwxyz*+";

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let matches = App::new("reg")
            .arg(Arg::new("print").short('p').long("print"))
            .arg(Arg::new("regfile").short('f').long("regfile"))
            .arg(Arg::new("reg"))
            .get_matches();
        
        if matches.is_present("regfile") {
            let fname = store::get_filename();
            println!("{}", fname);
            Command::new("vi").arg(fname).status().expect("Something went wrong.") ;

        } else if matches.is_present("print") {
            print_regfile()
        }

        if let Some(reg) = matches.value_of("reg") {

            let regs = store::get_regs();
            let lines: Vec<String> = reg.chars().map(|c| REGS.find(c).expect(&format!("bad reg: '{}'", c)))
                .map(|idx| regs.get(regs.len() - idx - 1).expect(&format!("index out of range, this should not happen")).to_owned())
                .collect();

            if lines.len() > 0 {
                let ln = lines.join(" ");
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(ln.to_owned()).unwrap();
                println!("{}", ln);
            }
        }
    } else {
        if let Some(line) = store::append_stdin() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(line.to_owned()).unwrap();
            print_regfile();
        }
    }
}

fn print_regfile() {
    store::get_regs().iter().rev().zip(REGS.chars()).for_each(|(r,c)| println!("\"{} {}",c, r));
}
