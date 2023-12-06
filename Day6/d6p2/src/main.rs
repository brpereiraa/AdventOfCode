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
    let mut sum:i32 = 0;
    let mut flag:i32 = 0;
    
    content = read_line("./input");

    let mut game_time = content[0].split(":").nth(1).unwrap().trim()
                            .split_whitespace().collect::<Vec<&str>>().join("").parse::<usize>().unwrap();
    let mut game_distance = content[1].split(":").nth(1).unwrap().trim()
                            .split_whitespace().collect::<Vec<&str>>().join("").parse::<usize>().unwrap();
    for i in 1..game_time {
        let time = game_time - i;
        if time * i > game_distance{
            sum = sum + 1;
        }
    }
    print!("Solutions: \n{sum}");

}
