extern crate clap;

use std::path::Path;
use std::fs;
use std::time::UNIX_EPOCH;
use clap::{App, Arg};

struct Options {
    name: String,
    is_show_all: bool,
    is_show_line: bool,
}

struct FileDetail {
    name: String,
    modified: Time,
}

#[derive(Debug)]
struct Time {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minutes: u64,
    seconds: u64,
}

impl Time {

    // TODO fix
    fn parse_sec(unixtime: u64) -> Time {
        let months = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let seconds = unixtime % 60;
        let minutes = unixtime / 60 % 60;
        let hour = unixtime / 60 / 60 % 24;
        let year = unixtime / 60 / 60 / 24 / 365;
        let mut day = unixtime / 60 / 60 / 24 % 365 - year / 4;

        let mut count = 0;
        for v in &months {
            if day > *v {
                day -= v;
                count += 1;
            }
        }

        Time {
            year: 1970 + year,
            month: 1 + count,
            day: 1 + day,
            hour: hour,
            minutes: minutes,
            seconds: seconds,
        }
    }

    fn format(&self, format: &str) -> String {
        format
            .replace("yyyy", &format!("{}", self.year))
            .replace("mm", &format!("{}", self.month))
            .replace("dd", &format!("{}", self.day))
            .replace("h", &format!("{}", self.hour))
            .replace("i", &format!("{}", self.minutes))
            .replace("s", &format!("{}", self.seconds))
            .to_string()
    }
}

#[test]
fn parse_sec_test() {
    let time1 = Time::parse_sec(1000000000);
    assert_eq!(time1.year, 2001);
    assert_eq!(time1.month, 9);
    assert_eq!(time1.day, 9);
    assert_eq!(time1.hour, 1);
    assert_eq!(time1.minutes, 46);
    assert_eq!(time1.seconds, 40);
}

fn main() {
    let app = App::new("ls")
        .arg(Arg::with_name("where"))
        .arg(Arg::with_name("all").help("show all").short("a"))
        .arg(Arg::with_name("line").help("show line").short("l"));
    let mathces = app.get_matches();
    let name = mathces.value_of("where").map_or_else(|| ".", |w| w);
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

    let mut files: Vec<FileDetail> = Vec::new();
    if path.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();

            let file_name = file.file_name();
            let name = file_name.to_str().unwrap();

            let metadata = file.metadata().unwrap();
            let sec = get_modified_sec(metadata);

            files.push(FileDetail {
                name: name.to_string(),
                modified: Time::parse_sec(sec),
            });
        }
    } else {
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        let metadata = path.metadata().unwrap();
        let sec = get_modified_sec(metadata);

        files.push(FileDetail {
            name: file_name,
            modified: Time::parse_sec(sec),
        });
    }

    let mut result = String::new();
    for file in files {
        if options.is_show_all || !file.name.starts_with(".") {
            if options.is_show_line {
                result += &format!("{} ", file.modified.format("mm dd h:i"));
            }
            result += &format!("{}{}", file.name, end);
        }
    }

    println!("{}", result);
}

fn get_modified_sec(metadata: fs::Metadata) -> u64 {
    let modified_time = metadata.modified().unwrap();
    modified_time.duration_since(UNIX_EPOCH).unwrap().as_secs()
}
