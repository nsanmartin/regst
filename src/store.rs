use std::io::{self, BufRead};

static NREGS: usize = 30;

pub fn get_filename() -> String {
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

pub fn append_stdin<'a>() -> Option<String> {
    let reg_fname = get_filename();
    let mut reglines = read_lines(&reg_fname);
    let mut stdin = stdin_to_lines();
    reglines.append(&mut stdin);
    let lines = &reglines.as_slice()[reglines.len()-std::cmp::min(NREGS, reglines.len())..];
    std::fs::write(&reg_fname, lines.join("\n")).expect("could not write regfile");
    if let Some(line) = lines.last() {
        return std::option::Option::Some(line.to_owned());
    } else {
        return None;
    }
}

fn read_lines(fname: &str) -> Vec<String> {
    let file = std::fs::File::open(fname).expect("Error opening file");
    let reader = std::io::BufReader::new(&file);
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>().unwrap();
    return lines;
}

pub fn get_regs() -> Vec<String> {
    read_lines(&get_filename())
}
