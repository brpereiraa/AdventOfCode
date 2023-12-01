use std::fs::read_to_string;

fn read_line(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }
    result
}

fn sum(line: &str) -> i32 {
    let mut result = String::new();
    let mut num = String::new();
    let mut flag = false;

    for c in line.chars(){
        if c.is_digit(10){
            if !flag {
                result.push(c);
                flag = true;
                num = c.to_string();
            }
            else {
                num = c.to_string();
            }
        }
    }
    result.push_str(&num);
    
    match result.parse::<i32>() {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Error parsing as integer: {}", err);
            0  // Return a default value or handle the error as needed
        }
    }
} 


fn main() {
    let mut bag = 0;
    let contents: Vec<String>;
    
    contents = read_line("./text.txt");
    for line in &contents {
        bag += sum(line);
    }
    println!("The final result is: {bag}");
}
