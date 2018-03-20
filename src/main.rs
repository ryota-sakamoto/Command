use std::path::Path;
use std::fs;

fn main() {
    println!("> ls .");
    ls(".");

    println!("> ls src");
    ls("src");
}

fn ls(p: &str) {
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

            // -a
            if !name.starts_with(".") {
                result += &format!("{}{}", name, end);
            }
        }
    }

    println!("{}", result);
}
