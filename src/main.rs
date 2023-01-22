mod bwt;
mod rotate;
mod count_runs;
mod cmp_by_spec_char;

use crate::bwt::bwt;
use crate::count_runs::count_runs;

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


