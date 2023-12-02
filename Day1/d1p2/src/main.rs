use std::fs::read_to_string;

fn read_line(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }
    result
}

fn line_word(line: &str, i:usize) -> String {
    let mut word = String::new();

    let mut word = String::new();
    let line_len = line.len();

    if i + 3 <= line_len && line[i..i + 3].contains("one") {
        word = "1".to_string();
    } else if i + 3 <= line_len && line[i..i + 3].contains("two") {
        word = "2".to_string();
    } else if i + 5 <= line_len && line[i..i + 5].contains("three") {
        word = "3".to_string();
    } else if i + 4 <= line_len && line[i..i + 4].contains("four") {
        word = "4".to_string();
    } else if i + 4 <= line_len && line[i..i + 4].contains("five") {
        word = "5".to_string();
    } else if i + 3 <= line_len && line[i..i + 3].contains("six") {
        word = "6".to_string();
    } else if i + 5 <= line_len && line[i..i + 5].contains("seven") {
        word = "7".to_string();
    } else if i + 5 <= line_len && line[i..i + 5].contains("eight") {
        word = "8".to_string();
    } else if i + 4 <= line_len && line[i..i + 4].contains("nine") {
        word = "9".to_string();
    }

	word
}

fn check_last_number(line: &str) -> String{
    if line.ends_with("one") {
        "1".to_string()
    } else if line.ends_with("two") {
        "2".to_string()
    } else if line.ends_with("three") {
        "3".to_string()
    } else if line.ends_with("four") {
        "4".to_string()
    } else if line.ends_with("five") {
        "5".to_string()
    } else if line.ends_with("six") {
        "6".to_string()
    } else if line.ends_with("seven") {
        "7".to_string()
    } else if line.ends_with("eight") {
        "8".to_string()
    } else if line.ends_with("nine") {
        "9".to_string()
    } else {
        String::new()
    }
}

fn first_value(line: &str) -> String {
	let mut num = String::new();
	let mut i: usize = 0;
	let mut word = String::new();

	for c in line.chars(){
		word = line_word(line, i);
		if word != "" {
			num = word;
			break;
		}
		if c.is_digit(10){
			num = c.to_string();
			break;
		}
		i += 1;
	}
	
	num
}

fn last_value(line: &str) -> String {
    let mut num = String::new();
	let mut i: usize = 0;
	let mut word = String::new();

    word = check_last_number(line);
    if word != "" {
        return word;
    }
	for c in line.chars(){
        word = line_word(line, i);
		if word != "" {
			num = word;
		}
		if c.is_digit(10){
			num = c.to_string();
		}
		i += 1;
	}
	num
}


fn main() {
    let mut bag: i32 = 0;
    let mut first = String::new();
    let mut second = String::new();
    let contents: Vec<String>;
    
    contents = read_line("./input");
    for line in &contents {
        first = first_value(line);
        second = last_value(line);
        first.push_str(&second);

        if let Ok(parsed_value) = first.parse::<i32>() {
            bag = bag + parsed_value;
            println!("Added {} to bag, new total: {}", parsed_value, bag);
        } else {
            println!("Failed to parse concatenated value: {}", first);
        }

    }
    println!("The final result is: {first}");
}
