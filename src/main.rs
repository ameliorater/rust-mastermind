use std::io;
use rand::Rng;
use std::panic::resume_unwind;
use std::ops::Index;

//notes
//broke on: 609800, 909099
//no digit reduction on: 870979

#[derive(Eq, PartialEq)]
struct Response {
    guess_code: Vec<u32>,
    right_place: u32, //# digits correct and in right place
    wrong_place: u32, //# digits correct and in the wrong place
}

fn main() {
    let num_choices = 10; //cannot be larger than 10
    let code_length = 6;

    loop {
        //set of all possible codes
        let mut remaining_codes: Vec<Vec<u32>> = generate_all_codes(num_choices, code_length);
        let mut same_boat_digits: Vec<Vec<u32>> = vec![];
        println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());

        //MANUAL CODE ENTRY
        let mut input = String::new();
        println!("\nPlease enter a secret code: ");
        io::stdin().read_line(&mut input).expect("Not a string");
        let input = input.trim(); //trim whitespace and save input
        let actual_code = string_to_vec(input);

        //AUTOMATIC CODE GENERATION
        //let actual_code = generate_code(num_choices, code_length);
        //print_code("Actual code: ", &actual_code);

//    //MANUAL GUESS ENTRY
//    loop {
//        let mut input = String::new();
//        println!("\nPlease enter a code guess: ");
//        io::stdin().read_line(&mut input).expect("Not a string");
//        let code_str = input.trim(); //trim whitespace and save input
//        let guess_code = string_to_vec(code_str);
//        let response = get_response(&actual_code, guess_code);
//        print_response(&response);
//
//        remaining_codes = remove_codes(remaining_codes, &response);
//        println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());
//    }

        //AUTOMATIC GUESSING
        let initial_guesses: Vec<Vec<u32>> = vec![num_to_vec(123456), num_to_vec(234567), num_to_vec(345678), num_to_vec(456789), num_to_vec(567890)];
        let mut previous_response = None;
        for guess_code in initial_guesses {
            print_vec("Guessed code: ", &guess_code);
            let response = get_response(&actual_code, guess_code);
            print_response(&response);
            remaining_codes = remove_codes(remaining_codes, &response);
            println!("{} {}", "Codes remaining before digit reduction: ", &mut remaining_codes.len());
            if let Some(previous_response) = previous_response {
                let tuple = reduce_digits(remaining_codes, &previous_response, &response, same_boat_digits);
                remaining_codes = tuple.0;
                same_boat_digits = tuple.1;
            }
            println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());
            print_vec_of_vec("Paired digits: ", &same_boat_digits);
            previous_response = Some(response);
        }

    //MANUAL LAST GUESS CODE ENTRY
    let mut input = String::new();
    println!("\nPlease enter a code guess: ");
    io::stdin().read_line(&mut input).expect("Not a string");
    let input = input.trim(); //trim whitespace and save input
    let guess_code = string_to_vec(input);
    let response = get_response(&actual_code, guess_code);
    print_response(&response);

    remaining_codes = remove_codes(remaining_codes, &response);
    let tuple =
        reduce_digits(remaining_codes, previous_response.as_ref().unwrap(), &response, same_boat_digits);
        remaining_codes = tuple.0;
        same_boat_digits = tuple.1;
        println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());

        println!("\n\n\n Remaining codes: ");
        for code in remaining_codes.iter() {
            for digit in code {
                print!("{}", digit);
            }
            print!("\n");
        }
    }
}

fn generate_code (num_choices: u32, code_length: u32) -> Vec<u32> {
    let mut code : Vec<u32> = Vec::new();
    let min = 0; //so we can have ten digits
    let max = num_choices;

    for i in 0..code_length {
        code.push(rand::thread_rng().gen_range(min, max));
    }
    code
}

fn get_response (actual_code: &Vec<u32>, guess_code : Vec<u32>) -> Response {
    let mut right_place = 0;
    let mut wrong_place = 0;

    for index in 0..actual_code.len() {
        if guess_code[index] == actual_code[index] {
            right_place += 1;
        } else if guess_code.contains(&actual_code[index]) {
            wrong_place += 1;
        }
    }
    Response { guess_code, right_place, wrong_place }
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
        if !responses_equal(&get_response(&codes[index], response.guess_code.clone()), &response) {
            codes.swap_remove(index);
        } else {
            index += 1;
        }
    }
    codes
}

fn reduce_digits (mut codes: Vec<Vec<u32>>, previous_response : &Response, response : &Response, mut same_boat_digits : Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    //find digit that IS in previous but not in current code
    //find digit that is NOT in previous but IS in current code
    //*if there is more than one digit added OR removed, return original codes array (no digits can be eliminated)
    //  if guesses do not fit this pattern, no digits can be eliminated, so return original codes Vec
    //  if they do fit this pattern, check if the sum went up, down, or stayed the same
    //    if sum went up,
    //    if sum went down,
    //    if sum stayed the same, add digit pair to same_boat_digits
    //also, check same_boat_digits before removing to see if extra digits can also be removed

    let mut removed_digit : u32 = 999; //initialize to impossible value for convenient checking
    let mut added_digit : u32 = 999;
    for digit in 0..=9 {
        if !previous_response.guess_code.contains(&digit) && response.guess_code.contains(&digit) {
            if added_digit == 999 {
                //no other added digits have been found, so this is the added digit
                added_digit = digit;
            } else {
                //another added digit has already been found, so we can't do digit reduction :(
                return (codes, same_boat_digits)
            }
        } else if previous_response.guess_code.contains(&digit) && !response.guess_code.contains(&digit) {
            if removed_digit == 999 {
                //no other removed digits have been found, so this is the added digit
                removed_digit = digit;
            } else {
                //another removed digit has already been found, so we can't do digit reduction :(
                return (codes, same_boat_digits)
            }
        }
    }
    //digit <-> digit swap did not occur, so don't remove codes
    //todo: could something be inferred if a digit was added but none were removed or v.v.?
    if removed_digit == 999 || added_digit == 999 {
        return (codes, same_boat_digits)
    }

    same_boat_digits = merge_boats(same_boat_digits);

    let previous_sum = previous_response.right_place + previous_response.wrong_place;
    let current_sum = response.right_place + response.wrong_place;
    let mut unused_digits : Vec<u32> = vec![];
    if current_sum > previous_sum {
        unused_digits.push(removed_digit);
        if let Some(matching_digits) = get_matching_digits(&same_boat_digits, &removed_digit) {
            for digit in matching_digits {
                if !unused_digits.contains(&digit) { unused_digits.push(digit) }
            }
            println!("YAY! Removed a matching digit via same-boat pair");
        }
        println!("{} {}", removed_digit, "(cycled out) was unused");
    } else if current_sum < previous_sum {
        unused_digits.push(added_digit);
        if let Some(matching_digits) = get_matching_digits(&same_boat_digits, &added_digit) {
            for digit in matching_digits {
                if !unused_digits.contains(&digit) { unused_digits.push(digit) }
            }
            unused_digits.push(matching_digit);
            println!("YAY! Removed a matching digit via same-boat pair");
        }
        println!("{} {}", added_digit, "(cycled in) was unused");
    } else { //sums are equal
        let digit_pair : Vec<u32> = vec![removed_digit, added_digit];
        same_boat_digits.push(digit_pair);
        println!("{} {} {} {}", removed_digit, "and", added_digit, "were added to a same-boat pair");
    }

    if unused_digits.len() > 0 {
        codes = remove_codes_with_digits(codes, &unused_digits);
    }
    (codes, same_boat_digits)
}

fn get_matching_digits(groups : &Vec<Vec<u32>>, digit : &u32) -> Option<Vec<u32>> {
    for group in groups {
        if group.contains(digit) {
            return Some(*group)
        }
    }
    return None
}

fn remove_codes_with_digits (mut codes: Vec<Vec<u32>>, digits: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut index = 0;
    loop {
        if index >= codes.len() {
            break
        }
        for digit in digits {
            if &codes[index].contains(digit) {
                codes.swap_remove(index);
            } else {
                index += 1;
            }
        }
    }
    codes
}

fn get_boat_index (same_boat_digits: &Vec<Vec<u32>>, check_digit : u32) -> Option<usize> {
    for (index, boat) in same_boat_digits.iter().enumerate() {
        for digit in boat {
            if *digit == check_digit {
                return Some(index)
            }
        }
    }
    None
}

fn merge_boats () {

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
    for digit_index in 0..code_length {
        highest_value_vec.push(num_choices-1); //-1 because starts at zero
    }
    vec_to_num(highest_value_vec)
}

fn num_to_vec (mut num: u32) -> Vec<u32> {
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

fn vec_to_num (mut vec: Vec<u32>) -> u32 {
    let mut num = 0;
    for (index, digit) in vec.iter().enumerate() {
        num += digit * 10_u32.pow(index as u32)
    }
    num
}

fn print_response (response : &Response) {
//    print!("Right place: ");
//    println!("{}", response.right_place);
//    print!("Wrong place: ");
//    println!("{}", response.wrong_place);
    println!("{} {} {} {} {}", "(", response.right_place, ", ", response.wrong_place, ")");
}

fn print_vec(label: &str, code : &Vec<u32>) {
    println!("{}", label);
    for element in code {
        print!("{}", element);
    }
    println!();
}

fn print_vec_of_vec(label: &str, code : &Vec<Vec<u32>>) {
    println!("{}", label);
    for element in code {
        for other_element in element {
            print!("{}", other_element);
        }
    }
    println!();
}
