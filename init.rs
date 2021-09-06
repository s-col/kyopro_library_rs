use std::io::{self, BufRead};

use lib::*;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn read_line_from_buf<T: BufRead>(input_line: &mut String, buf: &mut T) {
    input_line.clear();
    buf.read_line(input_line).unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
}

mod lib {}
