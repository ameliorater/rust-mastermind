use std::io;
use rand::Rng;
use std::panic::resume_unwind;
use std::ops::Index;
use std::collections::HashMap;
use std::borrow::Borrow;

//notes
//broke on: 609800, 909099
//no digit reduction on: 870979
//broke on: 986455
//broke on: 833907,

//12/13: broke on: 459155, 584699, 552202
//zero codes on: 034932, 711052

//12-16 broke on: 005432

#[derive(Eq, PartialEq, Clone)]
struct Response {
    guess_code: Vec<u32>,
    right_place: u32, //# digits correct and in right place
    wrong_place: u32, //# digits correct and in the wrong place
}

fn main() {
    let num_choices = 10; //cannot be larger than 10
    let code_length = 6;

    let mut codes_and_guess_totals : HashMap<Vec<u32>, u32> = HashMap::new();
    let mut games_played = 0;

    while games_played < 10 {
        let mut total_guesses = 0;
        let mut automatic_mode = false;

        //set of all possible codes
        let mut remaining_codes: Vec<Vec<u32>> = generate_all_codes(num_choices, code_length);
        //let mut same_boat_digits: Vec<Vec<u32>> = vec![];
        let mut same_boat_digits : HashMap<u32, u32> = HashMap::new();
        println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());

        //MANUAL CODE ENTRY
        let mut input = String::new();
        println!("\nPlease enter a secret code: \nIf you would like the computer to automatically calculate responses,\nadd an 'A' at the end of your code");
        io::stdin().read_line(&mut input).expect("Not a string");
        let mut input = input.trim(); //trim whitespace and save input
        let mut temp_string = input.replace("", "");
        if input.contains("A") {
            temp_string = input.replace("A", "");
            automatic_mode = true;
        }
        input = temp_string.as_ref();
        let actual_code = string_to_vec(input);

//        //AUTOMATIC CODE GENERATION
//        let actual_code = generate_code(num_choices, code_length);
//        print_vec("Actual code: ", &actual_code);

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
        let mut response = Response { guess_code: vec![0, 0, 0, 0, 0], right_place: 0, wrong_place: 0};
        for guess_code in initial_guesses {
            if remaining_codes.len() <= 1 {
                break
            }
            total_guesses += 1;
            print_vec("Guess: ", &guess_code);
            if automatic_mode {
                response = get_response(&actual_code, guess_code);
                print_response(&response);
            } else {
                loop {
                    let mut input = String::new();
                    println!("\nPlease enter a response in the form:\n\
                    First digit: number of digits correct and in the right place\n\
                    Second digit: number of digits correct and in the wrong place\n\
                    Example: 60 for the guess matching your code exactly");
                    io::stdin().read_line(&mut input).expect("Not a string");
                    let input = input.trim(); //trim whitespace and save input
                    if let Some(response) = string_to_response(input, &guess_code) {
                        break
                    } else {
                        println!("Invalid input, please try again");
                        continue
                    }
                }
            }
            remaining_codes = remove_codes(remaining_codes, &(response.clone()));
            println!("{} {}", "Codes remaining before digit reduction: ", &mut remaining_codes.len());
            if let Some(previous_response) = previous_response {
                let tuple = reduce_digits(remaining_codes, &previous_response, &response, same_boat_digits);
                remaining_codes = tuple.0;
                same_boat_digits = tuple.1;
            }
            println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());
            println!("{} {:?}", "Paired digits: ", &same_boat_digits);
            previous_response = Some(response.clone());
            println!("{} {}", "Code still in list?", remaining_codes.contains(&actual_code));
        }

        //MANUAL LAST GUESS CODE ENTRY
//        let mut input = String::new();
//        println!("\nPlease enter a code guess: ");
//        io::stdin().read_line(&mut input).expect("Not a string");
//        let input = input.trim(); //trim whitespace and save input
//        let guess_code = string_to_vec(input);
//        let response = get_response(&actual_code, guess_code);
//        print_response(&response);

        //PRINT ALL REMAINING CODES
        println!("\n\n\n Remaining codes: ");
        for code in remaining_codes.iter() {
            for digit in code {
                print!("{}", digit);
            }
            print!("\n");
        }

        //AUTOMATIC LAST GUESS
        let guess_code = get_sixth_guess(&same_boat_digits);
        print_vec("Guessed: ", &guess_code);
        let response = get_response(&actual_code, guess_code);
        print_response(&response);
        total_guesses += 1;

        remaining_codes = remove_codes(remaining_codes, &response);
        println!("{} {}", "Codes remaining before digit reduction: ", &mut remaining_codes.len());
        let tuple =
        reduce_digits(remaining_codes, previous_response.as_ref().unwrap(), &response, same_boat_digits);
        remaining_codes = tuple.0;
        same_boat_digits = tuple.1;
        println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());

        //PRINT ALL REMAINING CODES
        println!("\n\n\n Remaining codes: ");
        for code in remaining_codes.iter() {
            for digit in code {
                print!("{}", digit);
            }
            print!("\n");
        }

        //AUTOMATIC RANDOM GUESSES
        while remaining_codes.len() > 1 {
            total_guesses += 1;
            let guess_code = guess_randomly_from_remaining(&remaining_codes);
            print_vec("Guessed: ", &guess_code);
            let response = get_response(&actual_code, guess_code);
            print_response(&response);
            remaining_codes = remove_codes(remaining_codes, &response);
            println!("{} {}", "Codes remaining: ", &mut remaining_codes.len());
        }

//        //MANUAL GUESS ENTRY LOOP
//        while remaining_codes.len() > 1 {
//            let mut input = String::new();
//            println!("\nPlease enter a code guess: ");
//            io::stdin().read_line(&mut input).expect("Not a string");
//            let input = input.trim(); //trim whitespace and save input
//            let guess_code = string_to_vec(input);
//            let response = get_response(&actual_code, guess_code);
//            print_response(&response);
//
//            println!("\n\n\n Remaining codes: ");
//            for code in remaining_codes.iter() {
//                for digit in code {
//                    print!("{}", digit);
//                }
//                print!("\n");
//            }
//        }

        //PRINT ANSWER
        println!("\n\n\n Last code remaining: ");
        for code in remaining_codes.iter() {
            for digit in code {
                print!("{}", digit);
            }
            print!("\n");
        }
        println!("{} {}", "Guesses: ", total_guesses);

        //multi-game stats
        codes_and_guess_totals.insert(actual_code, total_guesses);
        games_played += 1;
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

fn string_to_response (input : &str, guess_code : &Vec<u32>) -> Option<Response> {
    //let input = input as String;
    if let Some(right_place) = input.chars().next() {
        let right_place_int = right_place as u32 - '0' as u32;
        if let Some(wrong_place) = input.chars().next() {
            let wrong_place_int = wrong_place as u32 - '0' as u32;
            Some(Response { guess_code: guess_code.clone(), right_place: right_place_int, wrong_place : wrong_place_int })
        } else {
            None
        }
    } else {
        None
    }
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

fn get_sixth_guess (same_boat_digits: &HashMap<u32, u32>) -> Vec<u32>{
    let mut guess = vec![6,7,8,9,0];
    for digit in 0..=9 {
        if let Some(_) = same_boat_digits.get(&digit) {
            if !guess.contains(&digit) {
                guess.push(digit);
                break
            }
        }
    }
    if guess.len() < 6 {
        guess.push(1)
    }
    guess
}

fn guess_randomly_from_remaining(remaining_codes: &Vec<Vec<u32>>) -> Vec<u32>{
    let rand_index = rand::thread_rng().gen_range(0, remaining_codes.len());
    return remaining_codes[rand_index].clone();
}

fn reduce_digits (mut codes: Vec<Vec<u32>>, previous_response : &Response, response : &Response, mut same_boat_digits : HashMap<u32, u32>) -> (Vec<Vec<u32>>, HashMap<u32, u32>) {
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

    //same_boat_digits = merge_boats(same_boat_digits);

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
            println!("YAY! Removed a matching digit via same-boat pair");
        }
        println!("{} {}", added_digit, "(cycled in) was unused");
    } else { //sums are equal
        //let digit_pair : Vec<u32> = vec![removed_digit, added_digit];
        //same_boat_digits.push(digit_pair);
        same_boat_digits = add_boat(same_boat_digits, &removed_digit, &added_digit);
        println!("{} {} {} {}", removed_digit, "and", added_digit, "were added to a same-boat pair");
    }

    if unused_digits.len() > 0 {
        codes = remove_codes_with_digits(codes, &unused_digits);
    }
    (codes, same_boat_digits)
}

fn get_matching_digits(groups : &HashMap<u32, u32>, digit : &u32) -> Option<Vec<u32>> {
    let mut matching_digits : Vec<u32> = vec![];
    if let Some(digit_group_id) = groups.get(digit) {
        for digit in 0..=9 {
            if groups.get(&digit) == Some(digit_group_id) {
                matching_digits.push(digit);
            }
        }
        if matching_digits.len() > 0 { return Some(matching_digits) }
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
            if index >= codes.len() {
                break
            }
            if codes[index].contains(digit) {
                codes.swap_remove(index);
                if index >= codes.len() {
                    break
                }
            } else {
                index += 1;
            }
        }
    }
    codes
}

fn get_boat_index (same_boat_digits: &Vec<Vec<u32>>, check_digit : &u32) -> Option<usize> {
    for (index, boat) in same_boat_digits.iter().enumerate() {
        for digit in boat {
            if digit == check_digit {
                return Some(index)
            }
        }
    }
    None
}

fn add_boat(mut same_boat_digits: HashMap<u32, u32>, digit_1: &u32, digit_2: &u32) -> HashMap<u32, u32> {
    if let Some(digit_1_group_id) = same_boat_digits.clone().get(digit_1) {
        if let Some(digit_2_group_id) = same_boat_digits.clone().get(digit_2) {
            //both digits present
            //need to change all matched with one key to the other key (will change all digit_2 grouped digits to digit_1 group)
            for digit in 0..=9 {
                if same_boat_digits.get(&digit) == Some(digit_2_group_id) {
                    same_boat_digits.insert(digit, *digit_1_group_id);
                }
            }
        } else {
            //digit 1 present but digit 2 not present
            same_boat_digits.insert(*digit_2, *digit_1_group_id);
        }
    } else if let Some(digit_2_group_id) = same_boat_digits.get(digit_2) {
        //digit 2 present but digit 1 not present
        same_boat_digits.insert(*digit_1, *digit_2_group_id);
    } else {
        //neither digit present
        let group_id = rand::thread_rng().gen_range(0, 1_000_000);
        same_boat_digits.insert(*digit_1, group_id);
        same_boat_digits.insert(*digit_2, group_id);
    }
    same_boat_digits
}

//fn merge_boats (mut same_boat_digits: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
//    //for (index, mut boat) in same_boat_digits.iter().enumerate() {
//    let mut index : usize = 0;
//    loop {
//        println!("Top of outer loop");
//        if index >= same_boat_digits.len() {
//            println!("Breaking outer loop");
//            break
//        }
//        //for digit in &mut same_boat_digits[index] {
//        let mut digit_index : usize = 0;
//        loop {
//            if digit_index >= same_boat_digits[index].len() {
//                println!("Breaking inner loop");
//                break
//            }
//            println!("{} {} {} {}", "length", same_boat_digits[index].len(), "digit index", digit_index);
//            let digit_to_check = &mut same_boat_digits[index][digit_index].clone();
//            if let Some(boat_index) = get_boat_index(&mut same_boat_digits, digit_to_check) {
//                println!("In SOMEthing");
//                let mut boat_to_append = &mut same_boat_digits[boat_index].clone();
//                same_boat_digits[index].append(boat_to_append);
//                same_boat_digits.remove(boat_index);
//                print_vec_of_vec("same boat digits", &same_boat_digits);
//            }
//            digit_index += 1;
//        }
//        index += 1;
//    }
//    same_boat_digits
//}

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
