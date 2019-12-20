use std::io;
use rand::Rng;
use std::collections::HashMap;
use std::panic::resume_unwind;
use std::hash::Hash;

#[macro_use]
extern crate derive_new;

#[derive(Eq, Hash, PartialEq, Clone, new)]
struct Response {
    right_place: u32, //# digits correct and in right place
    wrong_place: u32, //# digits correct and in the wrong place
}

fn get_sample (codes : &Vec<Vec<u32>>, length: usize) -> Vec<Vec<u32>> {
    let mut sample_codes: Vec<Vec<u32>> = Vec::new();
    let indices = get_random_ints(length, codes.len());
    for i in 0..codes.len() {
        if indices.contains(&i) {
            sample_codes.push(codes[i].clone())
        }
    }
    sample_codes
}

fn get_random_ints (length: usize, max: usize) -> Vec<usize> {
    let mut return_list = Vec::new();
    for i in 0..length {
        return_list[i] = rand::thread_rng().gen_range(0, max)
    }
    return_list
}

fn get_all_responses () -> Vec<Response> {
    let mut responses = Vec::new();
    for n in 0..6 {
        for i in 0..(6-n) {
            responses.push( Response{ right_place: n, wrong_place: i } )
        }
    }
    responses
}


fn main() {
    let sample_size = 1000;
    let num_choices = 10;
    let code_length = 6;

    //AUTOMATIC CODE GENERATION
    let actual_code = generate_code(num_choices, code_length);
    print_vec("Actual code: ", &actual_code);


    let mut score: HashMap<Vec<u32>, u32> = HashMap::new(); //keys are guesses, values are scores
    let mut remaining_codes: Vec<Vec<u32>> = generate_all_codes(num_choices, code_length);

    let guess_sample = get_sample(&remaining_codes, sample_size);
    //should answer_sample be the same as guess_sample?
    let answer_sample = get_sample(&remaining_codes, sample_size);

    for guess in guess_sample {
        let mut freq: HashMap<Response, u32> = HashMap::new();
        for answer in &answer_sample {
            let freq_key = get_response(answer, &guess);
            if let Some(freq_val) = freq.get(&freq_key) {
                freq.insert(freq_key, freq_val + 1);
            } else {
                freq.insert(freq_key, 1);
            }
        }

        let mut elim: HashMap<Response, u32> = HashMap::new();
        for response in get_all_responses() {
            for answer in &answer_sample {
                if get_response(&guess, answer) != response {
                    *elim.get_mut(&response).unwrap() += 1;
                }
            }
        }

        let mut sum = 0;
        for response in get_all_responses() {
            sum += freq[&response] * elim[&response]
        }
        score.insert(guess, sum);
    }
}


fn generate_code (num_choices: u32, code_length: u32) -> Vec<u32> {
    let mut code : Vec<u32> = Vec::new();
    let min = 0; //so we can have ten digits
    let max = num_choices;

    for _ in 0..code_length {
        code.push(rand::thread_rng().gen_range(min, max));
    }
    code
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
    Response { right_place, wrong_place }
}

fn string_to_response (input : &str, guess_code : &Vec<u32>) -> Response {
    let right_place = &input[0..1];
    let right_place = right_place.chars().next().unwrap();
    let right_place : u32 = right_place as u32 - '0' as u32;

    let wrong_place = &input[1..2];
    let wrong_place = wrong_place.chars().next().unwrap();
    let wrong_place = wrong_place as u32 - '0' as u32;

    Response { right_place, wrong_place }
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

fn generate_all_codes (num_choices: u32, code_length: u32) -> Vec <Vec<u32>> {
    let numeric_codes : Vec<u32> = (0..=get_highest_value_code_num(num_choices, code_length)).map(|i| i as u32).collect();
    let mut vector_codes : Vec<Vec<u32>> = Vec::new();
    for code in numeric_codes {
        //add vector of code digits to our vector of all codes
        let mut code_vec : Vec<u32> = num_to_vec(code);
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

fn print_vec_of_vec(label: &str, code : &Vec<Vec<u32>>) {
    println!("{}", label);
    for element in code {
        for other_element in element {
            print!("{}", other_element);
        }
    }
    println!();
}
