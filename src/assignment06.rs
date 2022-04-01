use rand::prelude::*;

pub fn run() {
    println!("Please enter Integer #");

    let stdin = std::io::stdin();
    let mut str_input = String::new();
    stdin
        .read_line(&mut str_input)
        .expect("Error Reading Input");

    //chop off the newline character
    str_input = String::from(str_input.trim());

    let size = match str_input.parse::<usize>() {
        Ok(n) => n,
        Err(e) => {
            println!("{str_input} is not an integer\n\n\n{e}");
            0
        }
    };

    let mut rng = thread_rng();
    let mut arr: Vec<bool> = vec![false; size];
    let mut count = 0;

    while !all_on(&arr) {
        match arr.get_mut(rng.gen_range(0..size)) {
            Some(n) => *n = !*n,
            None => (),
        }
        count += 1;
    }

    println!("It took {count} turns to for {size} switches to all be on\n\n");
}

fn all_on(arr: &Vec<bool>) -> bool {
    for n in arr.iter() {
        if !*n {
            return false;
        }
    }
    true
}
