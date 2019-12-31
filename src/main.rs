use std::{io, cmp};
use rand::Rng;
use std::collections::HashMap;

#[macro_use]
extern crate derive_new;

#[derive(Eq, PartialEq, Clone, new)]
struct Response {
    right_place: u32, //# digits correct and in right place
    wrong_place: u32, //# digits correct and in the wrong place
}
impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.right_place, self.wrong_place)
    }
}

fn main() {
    let num_choices = 10; //cannot be larger than 10
    let code_length = 6;
    let number_of_games = 10; //how many games to play before quitting program

    let mut codes_and_guess_totals : HashMap<Vec<u32>, u32> = HashMap::new();
    let mut games_played = 0;

    let all_correct_response = Response {right_place: 6, wrong_place: 0};

    //println!("response test: {}", get_response(&num_to_vec(491830), &num_to_vec(968334)));

    while games_played < number_of_games {
        let mut total_guesses = 0;
        let mut automatic_mode = true;

        //set of all possible codes
        let mut remaining_codes: Vec<Vec<u32>> = generate_all_codes(num_choices, code_length);

        //MANUAL CODE ENTRY
        let mut input = String::new();
        println!("\nPlease come up with a six-digit secret code. \n\
        You may use the digits 0 through 9. \nIf you trust the computer\
        , you may enter your code here.\n\
        If you'd like to play normally, press enter to continue.");
        io::stdin().read_line(&mut input).expect("Not a string");
        let input = input.trim(); //trim whitespace and save input
        if input == "" {
            //player pressed enter and would like to enter responses manually
            automatic_mode = false;
        }
        let actual_code = string_to_vec(input);

        //AUTOMATIC RANDOM GUESSES
        while remaining_codes.len() > 0 {
            total_guesses += 1;
            let guess_code = guess_randomly_from_remaining(&remaining_codes);
            print_vec("Guessed: ", &guess_code);

            let response;
            if automatic_mode {
                response = get_response(&actual_code, &guess_code);
                print_response(&response);
            } else {
                response = get_user_response();
            }

            if response == all_correct_response {
                break
            }

            remaining_codes = remove_codes(remaining_codes, &guess_code, &response);
            println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());

            if remaining_codes.len() < 20 {
                for code in &remaining_codes {
                    //print_vec("code: ", code);
                }
            }
        }

        if remaining_codes.len() >= 1 {
            println!("{} {} {}", "I guessed it!\nGuesses: ", total_guesses, "\n");
            //multi-game stats
            codes_and_guess_totals.insert(actual_code, total_guesses);
            games_played += 1;
        } else {
            println!("Your responses were not consistent (no codes are possible)\n\
            Please try playing again")
        }
    }

    println!("{:?}", codes_and_guess_totals);
    let mut total_guesses_all = 0;
    let mut count = 0;
    for element in codes_and_guess_totals.values() {
        total_guesses_all += *element;
        count += 1;
    }
    println!("{} {}", "Average guesses: ", total_guesses_all as f64/count as f64);
}

//491830
//968334

//fn get_response (actual_code: &Vec<u32>, guess_code : &Vec<u32>) -> Response {
//    let mut right_place = 0;
//    let mut wrong_place = 0;
//    let mut digits_used : HashMap<u32, bool> = HashMap::new();
//    //initialize all to false
//    for digit in 0..=9 {
//        digits_used.insert(digit, false);
//    }
//
//    for index in 0..actual_code.len() {
//        if guess_code[index] == actual_code[index] {
//            right_place += 1;
//        } else if actual_code.contains(&guess_code[index]) && !digits_used[&guess_code[index]] {
//            wrong_place += 1;
//            digits_used.insert(guess_code[index], true);
//        }
//    }
//    Response { right_place, wrong_place }
//}

fn get_response (actual_code: &Vec<u32>, guess_code : &Vec<u32>) -> Response {
    let mut actual_digit_indices: Vec<Vec<u32>> = vec![vec![]; 10]; //indices of first vec are digits 0-9
    let mut guess_digit_indices: Vec<Vec<u32>> = vec![vec![]; 10]; //indices of first vec are digits 0-9

    for index in 0..6 {
        //add this index to the list of indices for whichever digit is present in each code at this index
        actual_digit_indices[actual_code[index] as usize].push(index as u32);
        guess_digit_indices[guess_code[index] as usize].push(index as u32);
    }

    let mut right_place: Vec<u32> = vec![0; 10];
    let mut wrong_place: Vec<u32> = vec![0; 10];
    for digit in 0..=9 {
        for index in &actual_digit_indices[digit] {
            if guess_digit_indices[digit].contains(index) {
                right_place[digit] += 1;
            }
        }
        let count_in_actual = actual_digit_indices[digit].len() as u32;
        let count_in_guess = guess_digit_indices[digit].len() as u32;
//        println!("right place: {}", right_place[digit]);
//        println!("min of counts: {}", cmp::min(count_in_actual, count_in_guess));
        wrong_place[digit] += cmp::min(count_in_actual, count_in_guess) - right_place[digit];
    }

    Response {right_place: right_place.iter().sum(), wrong_place: wrong_place.iter().sum()}
}

fn get_user_response() -> Response {
    println!("Please enter your response: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a string");
    let input = input.trim(); //trim whitespace
    string_to_response(input)
}

fn string_to_response (input : &str) -> Response {
    let right_place = &input[0..1];
    let right_place = right_place.chars().next().unwrap();
    let right_place : u32 = right_place as u32 - '0' as u32;

    let wrong_place = &input[1..2];
    let wrong_place = wrong_place.chars().next().unwrap();
    let wrong_place = wrong_place as u32 - '0' as u32;

    Response { right_place, wrong_place }
}

fn remove_codes (mut codes: Vec<Vec<u32>>, guess_code: &Vec<u32>, response: &Response) -> Vec<Vec<u32>> {
    let mut num_loop_execs = 0;
    let mut index = 0;
    loop {
        num_loop_execs += 1;
        if index >= codes.len() {
            break
        }
        if get_response(&codes[index], guess_code) != *response {
            codes.swap_remove(index);
        } else {
            index += 1;
        }
    }
    println!("num loop execs: {}", num_loop_execs);
    codes
}

fn guess_randomly_from_remaining(remaining_codes: &Vec<Vec<u32>>) -> Vec<u32>{
    let rand_index = rand::thread_rng().gen_range(0, remaining_codes.len());
    return remaining_codes[rand_index].clone();
}

fn generate_all_codes (num_choices: u32, code_length: u32) -> Vec <Vec<u32>> {
    let numeric_codes : Vec<u32> = (0..=get_highest_value_code_num(num_choices, code_length)).map(|i| i as u32).collect();
    let mut vector_codes : Vec<Vec<u32>> = Vec::new();
    for code in numeric_codes {
        //add vector of code digits to our vector of all codes
        let mut code_vec = num_to_vec(code);
         //pad with zeros in the front if not long enough
        for i in 0..code_length - code.to_string().len() as u32 {
            code_vec.insert(i as usize, 0 as u32)
        }
        vector_codes.push(code_vec);
    }
    vector_codes
}

fn get_highest_value_code_num (num_choices: u32, code_length: u32) -> u32 {
    let mut highest_value_vec : Vec<u32> = Vec::new();
    for _ in 0..code_length {
        highest_value_vec.push(num_choices-1); //-1 because starts at zero
    }
    vec_to_num(highest_value_vec)
}

fn num_to_vec (num: u32) -> Vec<u32> {
    let mut vec : Vec<u32> = Vec::new();
    let mut remaining_num = num;

    loop {
        vec.push(remaining_num % 10);
        if remaining_num < 10 { break }
        remaining_num = remaining_num/10;
    }
    vec.reverse(); //because last digit is in first position
    vec
}

fn string_to_vec (input_str : &str) -> Vec<u32> {
    let mut vec : Vec<u32> = Vec::new();
    for char in input_str.chars() {
        vec.push(char as u32 - '0' as u32);
    }
    vec
}

fn vec_to_num (vec: Vec<u32>) -> u32 {
    let mut num = 0;
    for (index, digit) in vec.iter().enumerate() {
        num += digit * 10_u32.pow(index as u32)
    }
    num
}

fn print_response (response : &Response) {
    println!("{} {} {}", "Response: ", response.right_place, response.wrong_place);
}

fn print_vec(label: &str, code : &Vec<u32>) {
    print!("{}", label);
    for element in code {
        print!("{}", element);
    }
    println!();
}