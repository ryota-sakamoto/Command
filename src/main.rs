extern crate clap;

use std::path::Path;
use std::fs;
use clap::{App, Arg};

fn main() {
    let app = App::new("ls").arg(Arg::with_name("all").help("show all").short("a"));
    let mathces = app.get_matches();
    let is_show_all = mathces.is_present("all");

    println!("> ls {}.", if is_show_all {
        "-a "
    } else {
        ""
    });
    ls(".", is_show_all);

    println!("> ls {}src", if is_show_all {
        "-a "
    } else {
        ""
    });
    ls("src", is_show_all);
}

fn ls(p: &str, is_show_all: bool) {
    let mut result = String::new();
    let end = if true {
        "\n"
    // -l
    } else {
        " "
    };

    let path = Path::new(p);
    if path.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();

            let file_name = file.file_name();
            let name = file_name.to_str().unwrap();

            if is_show_all || !name.starts_with(".") {
                result += &format!("{}{}", name, end);
            }
        }
    }

    println!("{}", result);
}
