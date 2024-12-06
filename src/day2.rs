use std::{cmp, fs::File, io::Read, path::PathBuf, process};

pub fn run() {
    // println!("{}", get_lists_from_files());
    let file_name = PathBuf::from("resource/day2-1.txt");

    if !file_name.exists() {
        println!("file not exists");

        process::exit(1);
    }

    let mut contents = String::new();

    let _ = match File::open(file_name) {
        Ok(mut c) => c.read_to_string(&mut contents).unwrap(),
        Err(e) => {
            println!("unable to open file due to {:?}", e);
            process::exit(1);
        }
    };

    println!("{:?}", part_one(&contents));
    println!("{:?}", part_two(&contents))
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    Some(
        reports
            .into_iter()
            .filter(|report| is_safe(report, false))
            .count() as u32,
    )
}

fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);

    Some(
        reports
            .into_iter()
            .filter(|report| is_safe(report, true))
            .count() as u32,
    )
}
fn is_safe(report: &Vec<u32>, try_remove_bad_level: bool) -> bool {
    assert!(report.len() > 2);

    #[derive(PartialEq)]
    enum ReportOrder {
        Ascending,
        Decending,
    }

    let expected_order = match report[0].cmp(&report[1]) {
        cmp::Ordering::Greater => ReportOrder::Decending,
        cmp::Ordering::Less => ReportOrder::Ascending,
        cmp::Ordering::Equal => {
            return is_safe_after_remove(report, try_remove_bad_level);
        }
    };

    for i in 0..report.len() - 1 {
        let left_level = report[i];
        let right_level = report[i + 1];

        let order = match left_level.cmp(&right_level) {
            cmp::Ordering::Greater => ReportOrder::Decending,
            cmp::Ordering::Less => ReportOrder::Ascending,
            cmp::Ordering::Equal => {
                return is_safe_after_remove(report, try_remove_bad_level);
            }
        };

        if order != expected_order {
            return is_safe_after_remove(report, try_remove_bad_level);
        }

        let diff = left_level.abs_diff(right_level);

        // if !range.contains(&diff) {
        //     return false;
        // }
        if !(diff >= 1 && diff <= 3) {
            return is_safe_after_remove(report, try_remove_bad_level);
        }
    }
    true
}

fn is_safe_after_remove(report: &Vec<u32>, try_remove_bad_level: bool) -> bool {
    if try_remove_bad_level {
        for i in 0..report.len() {
            let mut left_removed_report = report.clone();
            left_removed_report.remove(i);
            if is_safe(&left_removed_report, false) {
                return true;
            }
        }
    }

    false
}
