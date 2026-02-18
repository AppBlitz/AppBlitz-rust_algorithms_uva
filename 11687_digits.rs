use std::io;
fn main() {
    let mut option = String::new();
    let mut number: isize;
    loop {
        option.clear();
        io::stdin().read_line(&mut option).unwrap();
        if option.trim() == "END" {
            break;
        } else {
            number = match option.trim().parse() {
                Ok(numbe) => numbe,
                Err(_) => -1,
            };
            println!("{}", index_v(number));
            continue;
        }
    }
}

fn index_v(number: isize) -> i64 {
    let auxiliary: i64;
    let mut auxiliary_number: isize = number;
    let mut vector_number: Vec<isize> = Vec::new();
    vector_number.push(auxiliary_number);
    let mut iterator: isize = 1;
    loop {
        auxiliary_number = counter_number(auxiliary_number);
        vector_number.push(auxiliary_number);
        if vector_number.len() >= 2
            && vector_number[(iterator - 1) as usize] == vector_number[iterator as usize]
        {
            auxiliary = iterator as i64;
            break;
        } else {
            iterator = iterator + 1;
            continue;
        }
    }
    return auxiliary;
}

fn counter_number(numbe: isize) -> isize {
    let mut number: isize = numbe;
    let mut auxiliary: isize = 0;
    while number != 0 {
        number = number / 10;
        auxiliary = auxiliary + 1;
    }
    return auxiliary;
}
