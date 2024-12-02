use std::{fs, path::Path};

pub trait IsSorted {
    fn is_sorted(&self) -> bool;
}

impl<T: PartialOrd> IsSorted for Vec<T> {
    fn is_sorted(&self) -> bool {
        self.windows(2).all(|w| w[0] <= w[1]) || self.windows(2).all(|w| w[0] >= w[1])
    }
}

fn parse_matrix(data: String) -> Vec<Vec<i32>> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let range = 1..=3;
    report
        .windows(2)
        .all(|level| range.contains(&(level[0] - level[1]).abs()))
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .into_iter()
        .filter(|report| report.is_sorted() && is_safe(&report))
        .count() as i32
}

fn count_safe_reports_dampener(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .into_iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                let mut r = (*report).clone();
                r.remove(i);
                r.is_sorted() && is_safe(&r)
            })
        })
        .count() as i32
}

fn main() {
    let data = match fs::read_to_string(Path::new("src/input1.txt")) {
        Ok(filecontent) => filecontent,
        Err(e) => panic!("Error while opening file: {e:?}"),
    };
    let parsed_reports = parse_matrix(data);
    let num_safe_reports = count_safe_reports(&parsed_reports);
    println!("The number of safe reports is {}", num_safe_reports);
    let num_safe_reports_dampener = count_safe_reports_dampener(&parsed_reports);
    println!(
        "The number of safe reports dampener is {}",
        num_safe_reports_dampener
    )
}
