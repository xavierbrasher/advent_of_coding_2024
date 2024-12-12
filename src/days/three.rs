use crate::common_functions::open_file;

pub fn main() {
    let input = open_file("inputs/day_three.txt");
    let line: String = input.split("\r\n").collect::<Vec<&str>>()[0].to_string();
    let mut mul_vals: Vec<i32> = Vec::new();

    println!("{:?}", line)
}
