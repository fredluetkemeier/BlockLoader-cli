use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("", "install", "Install a mod", "NAME");

    let matches = match opts.parse(&args[1..]) {
        Ok(val) => val,
        Err(err) => {
            panic!("{}", err)
        }
    };
}
