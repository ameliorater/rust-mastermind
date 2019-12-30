use std::io;
use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;

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
    let number_of_games = 0; //how many games to play before quitting program

    let mut codes_and_guess_totals : HashMap<Vec<u32>, u32> = HashMap::new();
    let mut games_played = 0;

    let subbed_digits = get_subbed_digits(&num_to_vec(123456), &num_to_vec(234567));
    println!("subbed in: {}, subbed out: {}", subbed_digits.0.unwrap(), subbed_digits.1.unwrap());

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

        let mut digit_info: HashMap<u32, u32> = HashMap::new();

        //RANDOM FIRST GUESS
        let guess_code = guess_randomly_from_remaining(&remaining_codes);
        let mut previous_response: Response = Response::new( vec![0,0,0,0,0,0], 0, 0);
        print_vec("Guessed: ", &guess_code);
        //TODO: clean up this duplicate code
        if automatic_mode {
            previous_response = get_response(&actual_code, &guess_code);
            print_response(&previous_response);
        } else {
            println!("Please enter your response: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Not a string");
            let input = input.trim(); //trim whitespace
            previous_response = string_to_response(input, &guess_code);
        }
        total_guesses += 1;

        //AUTOMATIC GUESSES
        while remaining_codes.len() > 1 {
            total_guesses += 1;

            let guess_code = get_next_guess(&remaining_codes, &digit_info, &previous_response);
            print_vec("Guessed: ", &guess_code);

            let response;
            if automatic_mode {
                response = get_response(&actual_code, &guess_code);
                print_response(&response);
            } else {
                println!("Please enter your response: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Not a string");
                let input = input.trim(); //trim whitespace
                response = string_to_response(input, &guess_code);
            }

            remaining_codes = remove_codes(remaining_codes, &response);
            println!("Codes remaining BEFORE digit elimination: {}", remaining_codes.len());

            //use digit elimination to update digit_info and further reduce remaining_codes
            digit_info = update_digit_info(&response, &previous_response, digit_info);
            remaining_codes = remove_codes_by_digit(remaining_codes, &digit_info);
            println!("Codes remaining AFTER digit elimination: {}", remaining_codes.len());
            previous_response = response;

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

fn get_response (actual_code: &Vec<u32>, guess_code : &Vec<u32>) -> Response {
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
    Response { guess_code: guess_code.clone(), right_place, wrong_place }
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

fn remove_codes (mut codes: Vec<Vec<u32>>, response: &Response) -> Vec<Vec<u32>> {
    let mut index = 0;
    loop {
        if index >= codes.len() {
            break
        }
        if !responses_equal(&get_response(&codes[index], &response.guess_code), response) {
            codes.swap_remove(index);
        } else {
            index += 1;
        }
    }
    codes
}

fn remove_codes_by_digit (mut codes: Vec<Vec<u32>>, digit_info: &HashMap<u32, u32>) -> Vec<Vec<u32>> {
    let mut unused_digits = Vec::new();
    for (digit, status) in digit_info {
        if *status == 0 as u32 { //digit is not used in actual code
            unused_digits.push(*digit);
        }
    }
    let mut index = 0;
    loop {
        if index >= codes.len() {
            break
        }
        let mut code_removed = false;
        for digit in &unused_digits {
            if let Some(code) = codes.get(index) {
                if code.contains(digit) {
                    codes.swap_remove(index);
                    code_removed = true;
                    continue
                }
            }
        }
        if !code_removed {
            index += 1;
        }
    }
    codes
}

fn update_digit_info (current_response: &Response, previous_response: &Response, mut digit_info: HashMap<u32, u32>) -> HashMap<u32, u32> {
    let (digit_subbed_in, digit_subbed_out)
        = get_subbed_digits(&current_response.guess_code, &previous_response.guess_code);

    //cannot add any digit info, return original map
    if digit_subbed_out == None || digit_subbed_in == None {
        return digit_info
    }

    let previous_sum = &previous_response.right_place + &previous_response.wrong_place;
    let current_sum = &current_response.right_place + &current_response.wrong_place;

    if current_sum > previous_sum {
        //sum increased -> subbed out digit is unused, subbed in digit is used
        digit_info = insert_digit(digit_info, &digit_subbed_in.unwrap(), 1);
        digit_info = insert_digit(digit_info, &digit_subbed_out.unwrap(), 0);
    }
    if current_sum < previous_sum {
        //sum decreased -> subbed out digit is used, subbed in digit is unused
        digit_info = insert_digit(digit_info, &digit_subbed_out.unwrap(), 1);
        digit_info = insert_digit(digit_info, &digit_subbed_in.unwrap(), 0);
    }
    if current_sum == previous_sum {
        //digits are in "same boat" (info gained about one will lead to info about the other)
        let rand_group_id = rand::thread_rng().gen_range(2, 2e31 as u32);
        digit_info.insert(digit_subbed_out.unwrap(), rand_group_id);
        digit_info.insert(digit_subbed_in.unwrap(), rand_group_id);
    }

    digit_info
}

fn insert_digit (mut digit_info: HashMap<u32, u32>, digit_to_insert: &u32, value_to_insert: u32) -> HashMap<u32, u32> {
    //check for grouped digits
    let mut matching_digit = *digit_to_insert; //default to this to avoid later problems
    let mut group_id: Option<u32> = None;

    if let Some(group_id_op) = digit_info.get(&digit_to_insert) {
        group_id = Some(*group_id_op);
    }

    if group_id != None {
        for (digit, status) in &digit_info {
            //if digit matches group_id and is not the original digit, insert both with new value
            if *status == group_id.unwrap() && *digit != *digit_to_insert {
                matching_digit = *digit;
            }
        }
    }

    digit_info.insert(*digit_to_insert, value_to_insert);
    digit_info.insert(matching_digit, value_to_insert);

    digit_info
}

//digit_info has digits as keys and status codes as values
// (0 for unused, 1 for used, random ID for "same boat"
fn get_next_guess (remaining_codes: &Vec<Vec<u32>>, digit_info: &HashMap<u32, u32>, previous_response: &Response) -> Vec<u32> {
    //find which digits would be good to get more info on
    let mut info_wanted_digits: Vec<u32> = Vec::new();
    for digit in 0..=9 as u32 {
        if !digit_info.contains_key(&digit) {
            info_wanted_digits.push(digit);
        }
    }

    let mut good_guesses = Vec::new();
    let mut better_guesses = Vec::new();
    for code in remaining_codes {
        let (subbed_in_digit, subbed_out_digit)
            = get_subbed_digits(code, &previous_response.guess_code);

        if subbed_in_digit == None || subbed_out_digit == None {
            //cannot use this code for digit elimination, keep looking
            continue
        } else {
            good_guesses.push(code.clone());

            if info_wanted_digits.contains(&subbed_in_digit.unwrap()) || info_wanted_digits.contains(&subbed_out_digit.unwrap()) {
                better_guesses.push(code.clone());
            }
            if info_wanted_digits.contains(&subbed_in_digit.unwrap()) && info_wanted_digits.contains(&subbed_out_digit.unwrap()) {
                //very good guess, return immediately to save time
                return code.clone()
            }
        }
    }

    if let Some(guess) = better_guesses.get(0) {
        return guess.clone()
    }
    if let Some(guess) = good_guesses.get(0) {
        return guess.clone()
    }

    //no good codes found, so guess randomly
    guess_randomly_from_remaining(remaining_codes)
}

fn guess_randomly_from_remaining(remaining_codes: &Vec<Vec<u32>>) -> Vec<u32>{
    let rand_index = rand::thread_rng().gen_range(0, remaining_codes.len());
    return remaining_codes[rand_index].clone();
}

//returns subbed in digit, subbed out digit (None if not present)
fn get_subbed_digits (current_code: &Vec<u32>, previous_code: &Vec<u32>) -> (Option<u32>, Option<u32>) {
    let mut digit_subbed_out = None;
    let mut digit_subbed_in = None;

    for digit in 0..=9 as u32 {
        let current_code_count = current_code.iter().filter(|&d| *d == digit).count();
        let previous_code_count = previous_code.iter().filter(|&d| *d == digit).count();

        //digits cannot appear multiple times in either code for elimination to be possible
        if current_code_count > 1 || previous_code_count > 1 {
            return (None, None)
        }

        //this digit was subbed in
        if previous_code_count + 1 == current_code_count {
            //another digit has already been subbed in, so this code will not work
            if digit_subbed_in != None {
                return (None, None)
            }
            digit_subbed_in = Some(digit);
        }
        //else if because a different digit must be subbed out than the one subbed in
        else if previous_code_count - 1 == current_code_count {
            //another digit has already been subbed out, so this code will not work
            if digit_subbed_in != None {
                return (None, None)
            }
            digit_subbed_out = Some(digit);
        }
    }

    (digit_subbed_in, digit_subbed_out)
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