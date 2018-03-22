extern crate clap;

use std::path::Path;
use std::fs;
use clap::{App, Arg};

struct Options {
    name: String,
    is_show_all: bool,
    is_show_line: bool,
}

fn main() {
    let app = App::new("ls")
        .arg(Arg::with_name("where"))
        .arg(Arg::with_name("all").help("show all").short("a"))
        .arg(Arg::with_name("line").help("show line").short("l"));
    let mathces = app.get_matches();
    let name = mathces.value_of("where").map_or_else(||".", |w|w);
    let is_show_all = mathces.is_present("all");
    let is_show_line = mathces.is_present("line");

    let options = Options {
        name: name.to_string(),
        is_show_all: is_show_all,
        is_show_line: is_show_line,
    };

    ls(&options);
}

fn ls(options: &Options) {
    let end = if options.is_show_line { "\n" } else { " " };
    let path = Path::new(&options.name);

    let mut files: Vec<String> = Vec::new();
    if path.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();

            let file_name = file.file_name();
            let name = file_name.to_str().unwrap();

            files.push(name.to_string());
        }
    } else {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        files.push(file_name);
    }

    let mut result = String::new();
    for file in files {
        if options.is_show_all || !file.starts_with(".") {
            result += &format!("{}{}", file, end);
        }
    }

    println!("{}", result);
}
