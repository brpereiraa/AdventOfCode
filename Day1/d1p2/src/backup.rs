use std::fs::read_to_string;

fn read_line(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines(){
        result.push(line.to_string())
    }
    result
}

fn line_word(line: &str, i:i8) -> &str {
	let mut word = "";

	if &line[i..4+i].find("one"){
		word = "1";
	} else if &line[i..4+i].find("two"){
		word = "2";
	} else if &line[i..6+i].find("three"){
		word = "3";
	} else if &line[i..5+i].find("four"){
		word = "4";
	} else if &line[i..5+i].find("five"){
		word = "5";
	} else if &line[i..4+i].find("six"){
		word = "6";
	} else if &line[i..6+i].find("seven"){
		word = "7";
	} else if &line[i..6+i].find("eight"){
		word = "8";
	} else if &line[i..5+i].find("nine"){
		word = "9";
	}

	word
}

fn first_value(line: &str) -> &str {
	let mut num: String::new();
	let mut i: i8 = 0;
	let mut word: String::new();

	for c in line.chars(){
		word = line_word(line, i);
		if word != "" {
			num = word;
			break;
		}
		if c.is_digit(10){
			num = c;
			break;
		}
		i++;
	}
	
	num
}

// fn sum(line: &str) -> i32 {
//     let mut result = String::new();
//     let mut num = String::new();
//     let mut flag = false;

//     for c in line.chars(){
//         if c.is_digit(10) || find_str(line){
//             if !flag {
//                 result.push(c);
//                 flag = true;
//                 num = c.to_string();
//             }
//             else {
//                 num = c.to_string();
//             }
//         }
//     }
//     result.push_str(&num);
    
//     match result.parse::<i32>() {
//         Ok(parsed) => parsed,
//         Err(err) => {
//             eprintln!("Error parsing as integer: {}", err);
//             0  // Return a default value or handle the error as needed
//         }
//     }
// } 


fn main() {
    let mut bag = 0;
    let contents: Vec<String>;
    
    contents = read_line("./text.txt");
    for line in &contents {
        bag = first(line);
		println!("The final result is: {bag}");

    }
    println!("The final result is: {bag}");
}
