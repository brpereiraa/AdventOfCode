use std::fs::read_to_string;

fn read_line(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }
    result
}

fn calc_value(line: &str) -> i32 {
    let mut numbers: Vec<i32>;
    let mut winning_n: Vec<i32>;
    let mut calc = 0;
    
    winning_n = line.split(":").nth(1).unwrap().split("|").next().unwrap()
                    .split_whitespace().map(|c| { c.parse::<i32>().unwrap()}).collect();
    numbers = line.split(":").nth(1).unwrap().split("|").nth(1).unwrap()
                    .split_whitespace().map(|c| { c.parse::<i32>().unwrap()}).collect();
    
    let num_matches: usize = winning_n.iter()
        .filter(|&num| numbers.contains(num))
        .count();
    
    if num_matches == 0{
        return calc;
    }

    calc = i32::pow(2, num_matches as u32 - 1);

    calc
}

fn main() {
    let contents: Vec<String>;
    let mut sum:i32 = 0;
    
    contents = read_line("./input");
    for line in &contents {
        sum += calc_value(line);
    }
    println!("{}", sum);
}
