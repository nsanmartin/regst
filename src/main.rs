extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::env;
use clap::{App, Arg};

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

static REGS: &str= "\"0123456789abcdefghijklmnopqrstuvwxyz*+";

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let matches = App::new("reg")
            .arg(Arg::new("print").short('p').long("print"))
            .arg(Arg::new("reg"))
            .get_matches();
        
        if matches.is_present("print") {
            store::get_regs().iter().rev().zip(REGS.chars()).for_each(|(r,c)| println!("\"{} {}",c, r));
        }

        if let Some(reg) = matches.value_of("reg") {
            if reg.len() != 1 {
                println!("Bad reg: {}", reg);
                panic!("reg should be a char, not '{}'", reg);
            }
            if let Some(idx) = REGS.find(reg) {
                if let Some(line) = store::get_regs().get(idx) {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    ctx.set_contents(line.to_owned()).unwrap();
                } else {
                    println!("Nothing in register {}", reg);
                }
            } else {
                println!("Bad reg: {}", reg);
                panic!("reg should be a char in {}, not '{}'", REGS, reg);
            }

        }
    } else {
        if let Some(line) = store::append_stdin() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(line.to_owned()).unwrap();
        }
    }
}
