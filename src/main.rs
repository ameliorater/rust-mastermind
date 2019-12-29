use std::io;
use rand::Rng;
use std::collections::HashMap;
#[macro_use]
extern crate derive_new;

#[derive(Eq, PartialEq, Clone, new)]
struct Response {
    guess_code: Vec<u32>,
    right_place: u32, //# digits correct and in right place
    wrong_place: u32, //# digits correct and in the wrong place
}

fn main() {
    let num_choices = 10; //cannot be larger than 10
    let code_length = 6;
    let number_of_games = 10; //how many games to play before quitting program

    let mut codes_and_guess_totals : HashMap<Vec<u32>, u32> = HashMap::new();
    let mut games_played = 0;

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
        while remaining_codes.len() > 1 {
            total_guesses += 1;
            let guess_code = guess_randomly_from_remaining(&remaining_codes);
            print_vec("Guessed: ", &guess_code);

            let response;
            if automatic_mode {
                response = get_response(&actual_code, guess_code);
                print_response(&response);
            } else {
                println!("Please enter your response: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Not a string");
                let input = input.trim(); //trim whitespace
                response = string_to_response(input, &guess_code);
            }

            remaining_codes = remove_codes(remaining_codes, response);
            //println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());
        }

        //PRINT ANSWER
        if remaining_codes.len() == 0 {
            println!("Your responses were not consistent (no codes are possible)\n\
            Please try playing again")
        } else {
            print!("\nYour code is: ");
            for code in remaining_codes.iter() {
                for digit in code {
                    print!("{}", digit);
                }
                print!("\n");
            }
            println!("{} {} {}", "Guesses: ", total_guesses, "\n");

            //multi-game stats
            codes_and_guess_totals.insert(actual_code, total_guesses);
            games_played += 1;
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

fn get_response (actual_code: &Vec<u32>, guess_code : Vec<u32>) -> Response {
    let mut right_place = 0;
    let mut wrong_place = 0;
    let mut digits_used : HashMap<u32, bool> = HashMap::new();
    //initialize all to false
    for digit in 0..=9 {
        digits_used.insert(digit, false);
    }

    for index in 0..actual_code.len() {
        if guess_code[index] == actual_code[index] {
            right_place += 1;
        } else if actual_code.contains(&guess_code[index]) && !digits_used[&guess_code[index]] { //changed 12-16
            wrong_place += 1;
            digits_used.insert(guess_code[index], true);
        }
    }
    Response { guess_code, right_place, wrong_place }
}

fn string_to_response (input : &str, guess_code : &Vec<u32>) -> Response {
    let right_place = &input[0..1];
    let right_place = right_place.chars().next().unwrap();
    let right_place : u32 = right_place as u32 - '0' as u32;

    let wrong_place = &input[1..2];
    let wrong_place = wrong_place.chars().next().unwrap();
    let wrong_place = wrong_place as u32 - '0' as u32;

    Response { guess_code: guess_code.clone(), right_place, wrong_place }
}

fn responses_equal (response1: &Response, response2 : &Response) -> bool {
    if response1.right_place == response2.right_place && response1.wrong_place == response2.wrong_place {
        return true
    }
    false
}

fn remove_codes (mut codes: Vec<Vec<u32>>, response: Response) -> Vec<Vec<u32>> {
    let mut index = 0;
    loop {
        if index >= codes.len() {
            break
        }
        if !responses_equal(&get_response(&codes[index], response.guess_code.clone()), &response) {
            codes.swap_remove(index);
        } else {
            index += 1;
        }
    }
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