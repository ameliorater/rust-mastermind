use std::io;
use rand::Rng;
use std::panic::resume_unwind;
use stable_vec::StableVec;

#[derive(Eq, PartialEq)]
struct Response {
    guess_code: Vec<u32>,
    right_place: u32, //# digits correct and in right place
    wrong_place: u32, //# digits correct and in the wrong place
}

fn main() {
    let num_choices = 10; //cannot be larger than 10
    let code_length = 4;

    let turn = 1;
    let won = false;


    //set of all possible codes
    let mut remaining_codes : StableVec<Vec<u32>> = generate_all_codes(num_choices, code_length);
    println!("{} {}", "Codes remaining: ", &mut remaining_codes.num_elements());

    let actual_code = generate_code(num_choices, code_length);

    loop {
        print_code(&actual_code);
        let mut input = String::new();
        println!("\nPlease enter a code guess: ");
        io::stdin().read_line(&mut input).expect("Not a string");
        let code_str = input.trim(); //trim whitespace and save input
        let guess_code = string_to_vec(code_str);
        let response = get_response(&actual_code, guess_code);
        print_response(&response);

        remaining_codes = remove_codes(remaining_codes, &response);
        println!("{} {}", "Codes remaining: ", &mut remaining_codes.num_elements());
    }

//    for code in remaining_codes.iter() {
//        for digit in code {
//            print!("{}", digit);
//        }
//        print!("\n");
//    }

    //print!("{}", get_highest_value_code_num(num_choices, code_length));

//    //print code
//    for value in code {
//        print!("{}", value)
//    }
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

fn print_response (response : &Response) {
    print!("Right place: ");
    println!("{}", response.right_place);
    print!("Wrong place: ");
    println!("{}", response.wrong_place);
}

fn responses_equal (response1: &Response, response2 : &Response) -> bool {
    if response1.right_place == response2.right_place && response1.wrong_place == response2.wrong_place {
        return true
    }
    false
}

fn remove_codes (mut codes: StableVec<Vec<u32>>, response: &Response) -> StableVec<Vec<u32>> {
    let mut index = 0;
    loop {
        if index >= codes.num_elements() {
            break
        }
        if !responses_equal(&get_response(&codes[index], response.guess_code.clone()), &response) {
            codes.remove(index);
        } else {
            index += 1;
        }
        if codes.num_elements() % 1000 == 0 {
            println!("{} {}", "Code list length: ", codes.num_elements());
        }
        //println!("{} {} {} {}", "index: ", index, "length: ", codes.len());
    }

    codes
}

fn generate_all_codes (num_choices: u32, code_length: u32) -> StableVec<Vec<u32>> {
    let numeric_codes : Vec<u32> = (0..=get_highest_value_code_num(num_choices, code_length)).map(|i| i as u32).collect();
    let mut vector_codes : StableVec<Vec<u32>> = StableVec::new();
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

fn get_highest_value_code_num (num_choices: u32, code_length: u32) -> u32 {
    let mut highest_value_vec : Vec<u32> = Vec::new();
    for digit_index in 0..code_length {
        highest_value_vec.push(num_choices-1); //-1 because starts at zero
    }
    vec_to_num(highest_value_vec)
}

fn print_code(code : &Vec<u32>) {
    println!("{}", "Actual code: ");
    for element in code {
        print!("{}", *element);
    }
}