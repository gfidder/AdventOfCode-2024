use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut a = App::new("day2-2/test_file.txt".to_string());
    a.parse_file();
    a.calculate_safe_reports();

    println!("Done!");
}

#[derive(Default)]
pub struct App {
    path_to_file: String,
    reports: Vec<Vec<i32>>,
    safe_reports: i32,
}

impl App {
    pub fn new(path_to_file: String) -> Self {
        Self {
            path_to_file,
            safe_reports: 0,
            ..Default::default()
        }
    }

    pub fn parse_file(&mut self) {
        let lines = self.read_lines(&self.path_to_file).unwrap();

        for line in lines.flatten() {
            let x: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            self.reports.push(x);
        }
    }

    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn calculate_safe_reports(&mut self) {
        for report in &self.reports {
            if self.determine_is_report_safe(report) {
                self.safe_reports += 1;
            }
        }

        println!("Number of safe reports: {}", self.safe_reports);
    }

    fn determine_is_report_safe(&self, report: &Vec<i32>) -> bool {
        let mut decreasing = false;
        let mut increasing = false;
        let report_size = report.len();
        let mut num_unsafe_levels = 0;
        for i in 0..(report_size - 1) {
            let l_val = report[i];
            let r_val = report[i + 1];

            let abs_difference = (l_val - r_val).abs();

            if abs_difference == 0 || abs_difference > 3 {
                return false;
            }
            if l_val > r_val {
                decreasing = true;

                if increasing {
                    return false;
                }
            } else {
                increasing = true;

                if decreasing {
                    return false;
                }
            }
        }

        true
    }
}
