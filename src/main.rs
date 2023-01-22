use std::cmp::Ordering;

const START_OF_TEXT: char = '\u{02}';
const END_OF_TEXT: char = '\u{03}';

fn main() {
    //add test database
    let input = [
        "banana",
    ];
    for s in input.iter() {
        let bwt = bwt(s);
        println!("Input: {}", s);
        println!("\tBWT: {}", bwt.replace(START_OF_TEXT, "^").replace(END_OF_TEXT, "|"));
        println!("{}",count_runs(&*bwt));
    }
}

fn bwt(input: &str) -> String {
    let mut table: Vec<String> = vec![];

    let mut input_string = format!("{}{}{}", START_OF_TEXT, input, END_OF_TEXT);

    //all possible rotations probably will be updated
    let mut i = input_string.len();

    while i > 0 {
        table.push(
            format!("{}",input_string)
        );
        input_string = rotate(&*input_string);
        i = i-1;
    }

    table.sort_by(|a, b| cmp_by_spec_char(&a, &b));

    table
        .iter()
        .map(|s| s.chars().nth_back(0).unwrap())
        .collect::<String>()
}

pub fn rotate(str: &str) -> String {
    let mut str_vec: Vec<char> = str.chars().collect();
    str_vec.rotate_right(1);
    str_vec.iter().collect()
}

fn count_runs(bwt: &str) -> i32 {
    let mut run_count = 1;
    let mut prev_char = bwt.chars().next().unwrap();

    for c in bwt.chars().skip(1) {
        if c != prev_char {
            run_count += 1;
        }
        prev_char = c;
    }

    return run_count;
}

pub fn cmp_by_spec_char(a: &str, b: &str) -> Ordering {
    let mut a_iter = a.chars();
    let mut b_iter = b.chars();

    while let (Some(a_char), Some(b_char)) = (a_iter.next(), b_iter.next()) {
        if a_char != b_char {
            return if (a_char == END_OF_TEXT || a_char == START_OF_TEXT) && !(b_char == END_OF_TEXT || b_char == START_OF_TEXT) {
                Ordering::Greater
            } else if (b_char == END_OF_TEXT || b_char == START_OF_TEXT) && !(a_char == END_OF_TEXT || a_char == START_OF_TEXT) {
                Ordering::Less
            } else {
                a_char.cmp(&b_char)
            }
        }
    }

    return if a.len() > b.len() {
        Ordering::Greater
    } else if a.len() < b.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

