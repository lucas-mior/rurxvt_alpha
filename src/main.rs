use nix::unistd::getppid;
use std::process::exit;
use std::env;
use std::path::Path;
use std::fs;
use std::cmp;

static NUMBERS: &'static [i32] = &[0, 10, 20, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 93, 96, 100];
static DEF_OPACITY: usize = 13;

fn main() {
    let argv: Vec<String> = env::args().collect();
    const NAME: &str = "opacity";
    const CACHE: &str = "/tmp";
    let window_id: i32;

    if argv.len() <= 1 {
        help();
        exit(1);
    } else {
        window_id = getppid().into();
    }

    let file = format!("{}/{}_{}", CACHE, NAME, window_id);

    let path = Path::new(&file);
    let mut current: usize;
    if path.exists() {
        let data = fs::read_to_string(&file).expect("Unable to read file");
        current = data.parse().unwrap();
    } else {
        current = DEF_OPACITY;
    }

    match argv[1].as_str() {
        "-" => current = cmp::max(current - 1, 0),
        "+" => current = cmp::min(current + 1, NUMBERS.len() - 1),
        "=" => current = NUMBERS.len() - 1,
        _ => { 
            help(); 
            exit(1);
        }
    };

    fs::write(file, current.to_string()).expect("Unable to write file");
    print!("\x1b]011;[{}]#000000\x07", NUMBERS[current]); //background
    print!("\x1b]708;[{}]#000000\x07", NUMBERS[current]); //border

    return;
}

fn help() {
    println!("rurxvt_opacity [-+=h]\n
             - -- decrease\n
             + -- increase\n
             = -- set 100% opaque\n
             h -- print this help message");
}