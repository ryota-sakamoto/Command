extern crate clap;

use std::path::Path;
use std::fs;
use clap::{App, Arg};

struct Options {
    is_show_all: bool,
    is_show_line: bool,
}

fn main() {
    let app = App::new("ls")
        .arg(Arg::with_name("all").help("show all").short("a"))
        .arg(Arg::with_name("line").help("show line").short("l"));
    let mathces = app.get_matches();
    let is_show_all = mathces.is_present("all");
    let is_show_line = mathces.is_present("line");

    let options = Options {
        is_show_all: is_show_all,
        is_show_line: is_show_line,
    };

    println!("> ls {}.", if options.is_show_all { "-a " } else { "" });
    ls(".", &options);

    println!("> ls {}src", if options.is_show_all { "-a " } else { "" });
    ls("src", &options);
}

fn ls(p: &str, options: &Options) {
    let mut result = String::new();
    let end = if options.is_show_line { "\n" } else { " " };

    let path = Path::new(p);
    if path.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();

            let file_name = file.file_name();
            let name = file_name.to_str().unwrap();

            if options.is_show_all || !name.starts_with(".") {
                result += &format!("{}{}", name, end);
            }
        }
    }

    println!("{}", result);
}
