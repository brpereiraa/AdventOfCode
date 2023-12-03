use std::fs::read_to_string;

fn read_line(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }
    result
}

fn info_check(info: Vec<&str>) -> bool{
    let mut result: bool = false;

    for part in info{
        // Split into different strings for each color.
        let game = part.split(",");
        for games in game {
            //Check in each color, if number is over limit
            if games.contains("red") {
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > 12{
                    result = true;
                } 
            } else if games.contains("blue") {
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > 14{
                    result = true;
                } 
            } else if games.contains("green") {
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > 13{
                    result = true;
                } 
            }
        }
    }

    result
}

fn get_info(line: &str) -> i32{
    let mut game_id: &str;
    let game_info: Vec<&str>;
    let mut bag: i32 = 0;

    // Get first part of string when splitting on : character.
    game_id = line.split(":").next().unwrap();
    game_id = game_id.split_whitespace().nth(1).unwrap();

    // Get game info after : character;
    game_info = line.split(":").nth(1).unwrap().split(";").collect();

    //CHeck if there is elements over the limit
    if !info_check(game_info) {
        bag = game_id.parse::<i32>().unwrap();
    };
    
    bag
}

fn main() {
    let contents: Vec<String>;
    let mut sum:i32 = 0;
    
    contents = read_line("./input");
    for line in &contents {
        sum += get_info(line);
    }
    println!("{sum}");
}
