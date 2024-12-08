use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process;
pub fn run() {
    let file_name = PathBuf::from("resource/day3-1.txt");

    if !file_name.exists() {
        println!("file not exists");

        process::exit(1);
    }

    let mut contents = String::new();

    let _ = match File::open(file_name) {
        Ok(mut f) => f.read_to_string(&mut contents).unwrap(),
        Err(e) => {
            println!("encountered error: {:?}", e);

            process::exit(1);
        }
    };

    println!("{}", part_one(&contents));
    println!("{}", part_two(&contents));
}

fn part_one(input: &String) -> u64 {
    let mut result: u64 = 0;

    let pattern = match regex::Regex::new(r#"mul\((\d+),(\d+)\)"#) {
        Ok(p) => p,
        Err(e) => {
            println!("error occurred while creating regex: {:?}", e);

            process::exit(1);
        }
    };

    for (_, [num1, num2]) in pattern.captures_iter(&input).map(|c| c.extract()) {
        let num1: u32 = num1.parse().unwrap();
        let num2: u32 = num2.parse().unwrap();

        result += (num1 * num2) as u64;
    }

    result
}

fn part_two(input: &String) -> u64 {
    let mut result: u64 = 0;

    let input = input.split("\n").collect::<Vec<&str>>().concat();

    let do_dont_pattern = match regex::RegexBuilder::new(
        r#"(^.*?mul\(\d+,\d+\).*?do\(\).*?mul\(\d+,\d+\).*?don't\(\))|(do\(\).*?mul\(\d+,\d+\).*?don't\(\))|(do\(\).*?mul\(\d+,\d+\).*?$)"#,
    ).multi_line(true).build() {
        Ok(p) => p,
        Err(e) => {
            println!("error occurred while creating regex: {:?}", e);

            process::exit(1);
        }
    };

    for val in do_dont_pattern.find_iter(&input) {
        let mul_pattern = match regex::Regex::new(r#"mul\((\d+),(\d+)\)"#) {
            Ok(p) => p,
            Err(e) => {
                println!("error occurred while creating regex: {:?}", e);

                process::exit(1);
            }
        };

        for (_, [num1, num2]) in mul_pattern.captures_iter(val.as_str()).map(|c| c.extract()) {
            let num1: u32 = num1.parse().unwrap();
            let num2: u32 = num2.parse().unwrap();

            result += (num1 * num2) as u64;
        }
    }

    result
}
