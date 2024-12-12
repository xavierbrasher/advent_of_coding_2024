use crate::common_functions::open_file;

fn check_valid(row: Vec<i32>) -> bool {
    let ascending = row[0] < row[1];

    for i in 1..row.len() {
        let difference = row[i] - row[i - 1];
        if (ascending && difference < 0) || (!ascending && difference > 0) || difference == 0 {
            return false;
        }

        if difference > 3 || difference < -3 {
            return false;
        }
    }

    return true;
}

pub fn main() {
    let input = open_file("inputs/day_two.txt");
    let lines: Vec<&str> = input.split("\r\n").collect();

    let mut valids = 0;
    let mut extended_valids = 0;

    for i in 0..lines.len() - 1 {
        let row: Vec<i32> = lines[i]
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if check_valid(row.clone()) {
            valids += 1;
            extended_valids += 1;
        } else {
            for i in 0..row.len() {
                let mut cloned = row.clone();
                cloned.remove(i);
                if check_valid(cloned) {
                    extended_valids += 1;
                    break;
                }
            }
        }
    }

    println!("Valids: {:?}", valids);
    println!("Problem Dampener Valids: {:?}", extended_valids)
}
