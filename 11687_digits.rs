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
            number = match option.trim().parse::<isize>() {
                Ok(numbe) => numbe,
                Err(_) => -1,
            };
            println!("{}", index_v(number));
            continue;
        }
    }
}

fn index_v(number: isize) -> i64 {
    let mut auxiliary: i64 = 1;
    let mut auxiliary_number: isize = number;
    loop {
        if auxiliary_number == counter_number(auxiliary_number) {
            break;
        } else {
            auxiliary_number = counter_number(auxiliary_number);
            auxiliary = auxiliary + 1;
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
