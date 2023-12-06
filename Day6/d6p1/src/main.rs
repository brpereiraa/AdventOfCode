use std::fs::read_to_string;

fn read_line(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }

    result
}

fn main() {
    let content: Vec<String>;
    let mut sum_last:i32 = 1;
    let mut flag:i32 = 0;
    
    content = read_line("./input");

    for i in 0..4 {
        let mut sum:i32 = 0;
        let mut game_time = content[0].split(":").nth(1).unwrap().split_whitespace().nth(i).unwrap().parse::<i32>().unwrap();
        let mut game_distance = content[1].split(":").nth(1).unwrap().split_whitespace().nth(i).unwrap().parse::<i32>().unwrap();
        for i in 1..game_time {
            let time = game_time - i;
            if time * i > game_distance{
                sum = sum + 1;
            }
        }
        sum_last = sum_last * sum;
    }
    print!("Solutions: {sum_last}.\n");

}
