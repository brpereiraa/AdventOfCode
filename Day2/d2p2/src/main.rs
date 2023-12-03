use std::fs::read_to_string;

fn read_line(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }

    result
}

fn info_check(info: Vec<&str>) -> i32{
    let mut result: i32 = 0;
    let mut blue_min: i32 = 1;
    let mut red_min: i32 = 1;
    let mut green_min: i32 = 1;


    for part in info {
        // Split into different strings for each color.
        let game = part.split(",");
        for games in game {
            //Check in each color, if number is over limit and if not, check for min needed
            if games.contains("red") {
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > 12{
                    result = 0
                }
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > red_min{
                    red_min = games.split_whitespace().next().unwrap().parse::<i32>().unwrap();
                } 
            } else if games.contains("blue") {
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > 14{
                    result = 0
                }
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > blue_min{
                    blue_min = games.split_whitespace().next().unwrap().parse::<i32>().unwrap();
                }  
            } else if games.contains("green") {
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > 13{
                    result = 0
                } 
                if games.split_whitespace().next().unwrap().parse::<i32>().unwrap() > green_min{
                    green_min = games.split_whitespace().next().unwrap().parse::<i32>().unwrap();
                } 
            }
        }
        
    }
    
    result = green_min * blue_min * red_min;

    result
}

fn get_info(line: &str) -> i32{
    let mut game_id: &str;
    let game_info: Vec<&str>;
    let mut bag: i32 = 0;

    // Get game info after : character;
    game_info = line.split(":").nth(1).unwrap().split(";").collect();

    //Check if there is elements over the limit
    bag = info_check(game_info);
    
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
