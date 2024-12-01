use std::{collections::HashMap, fs::File, io::Read, iter::zip, path::PathBuf, process};

pub fn run() {
    let (mut left_nums, mut right_nums) = get_lists_from_files();
    let diff = diff(&mut right_nums, &mut left_nums);
    println!("{}", diff);
    let sim = similarity(left_nums, right_nums);
    println!("{}", sim);
}

fn diff(right_nums: &mut Vec<u32>, left_nums: &mut Vec<u32>) -> u64 {
    left_nums.sort();
    right_nums.sort();

    let mut res: u64 = 0;

    for (left, right) in zip(left_nums, right_nums) {
        res += left.abs_diff(*right) as u64;
    }

    res
}

fn get_lists_from_files() -> (Vec<u32>, Vec<u32>) {
    let file_name = PathBuf::from("resource/day1-1.txt");

    if !file_name.exists() {
        println!("file not exists");

        process::exit(1);
    }

    let mut right_nums = Vec::new();
    let mut left_nums = Vec::new();

    let mut contents = String::new();

    let _ = match File::open(file_name) {
        Ok(mut c) => c.read_to_string(&mut contents).unwrap(),
        Err(e) => {
            println!("unable to open file due to {:?}", e);
            process::exit(1);
        }
    };

    for line in contents.lines() {
        let mut splitted_line = line.split("   ").into_iter();
        let left = splitted_line.next();
        if let Some(l) = left {
            let left: u32 = l.parse().unwrap();
            left_nums.push(left);
        }
        let right = splitted_line.next();
        if let Some(r) = right {
            let right: u32 = r.parse().unwrap();
            right_nums.push(right);
        }
    }
    (left_nums, right_nums)
}

fn similarity(left_nums: Vec<u32>, right_nums: Vec<u32>) -> u64 {
    let mut res: u64 = 0;
    let right_nums_count: HashMap<u32, u16> = HashMap::new();

    let right_nums_count = right_nums.into_iter().fold(right_nums_count, |mut acc, x| {
        let val = acc.entry(x).or_insert(0);
        *val += 1;
        acc
    });

    for num in left_nums {
        let val = *right_nums_count.get(&num).unwrap_or(&0);
        res += (num * val as u32) as u64;
    }

    res
}
