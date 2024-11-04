use picross_solver::display::*;

extern crate getopts;

use getopts::Options;
use std::env;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("c", "cross_test", "Draw a cross example", "SIZE");
    opts.optopt("d", "draw_picross", "Draw a picross example", "SIZE");
    opts.optflag("h", "help", "Print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("c") {
        let size_str = matches.opt_str("c"); 
        let size  = match size_str {
            Some(x) => x.parse().unwrap(),
            None => 10 ,
        };
        let img = create_cross_image_board(size);
        img.display();
    }
    if matches.opt_present("d") {
        let size_str = matches.opt_str("d"); 
        let size  = match size_str {
            Some(x) => x.parse().unwrap(),
            None => 5 ,
        };        
        let picross = PicrossBoard::new_empty(size, size);
        picross.display();
    }
}
