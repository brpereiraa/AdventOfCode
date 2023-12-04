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

    for num in winning_n {
        println!("Winning numbers:{}", num);
    }
    for num in numbers {
        println!("Numbers:{}", num);
    }
    println!("\n");
    calc
}

fn main() {
    let contents: Vec<String>;
    let mut sum:i32 = 0;
    
    contents = read_line("./input");
    for line in &contents {
        calc_value(line);
        // println!("{}", line);
    }
}
