use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let mut a = App::new("day1-2/input_file.txt".to_string());
    a.parse_file();
    a.calculate_results();

    println!("Done!");
}

#[derive(Default)]
pub struct App {
    path_to_file: String,
    left_columns: HashSet<i32>,
    right_columns: Vec<i32>,
}

impl App {
    pub fn new(path_to_file: String) -> Self {
        Self {
            path_to_file,
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
            self.left_columns.insert(x[0]);
            self.right_columns.push(x[1]);
        }
    }

    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn calculate_results(&self) {
        // let size = self.left_columns.len();

        // let mut results = Vec::new();

        let mut sum = 0;

        for value in &self.left_columns {
            let mut count = 0;

            for right_val in &self.right_columns {
                if value == right_val {
                    count += 1;
                }
            }

            let x = value * count;
            sum += x;
        }

        // for i in 0..size {
        //     let left_value = self.left_columns[i];
        //     let right_value = self.right_columns[i];

        //     let distance = (left_value - right_value).abs();

        //     results.push(distance);
        // }

        // let sum: i32 = results.iter().sum();

        println!("The total sum it: {}", sum);
    }
}
