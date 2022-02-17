use std::io;

pub fn run(){
    println!("Assignment 2\n");
    println!("Josh Johnson");
    let stdin = io::stdin();
    let mut str_input: String;
    let mut arr: [i64; 3] = [0; 3];

    for i in 0..3 {
        println!("Please enter Integer #{}", i + 1);

        str_input = String::new();
        stdin.read_line(&mut str_input).expect("Error Reading Input");

        //chop off the newline character
        str_input = String::from(str_input.trim());

        match str_input.parse::<i64>() {
            Ok(n) => {
                arr[i] = n;
            }
            Err(e) => {
                println!("{str_input} is not an integer\n{e}");
                return;
            }
        }
    }
    //println!("You entered: {:?}\nand more importantly, the code works.", arr);

    let mut sum: i64 = 0;
    let mut prod: i64 = 1;
    for n in arr {
        sum += n;
        prod *= n;
    }
    let avg: f64 = (sum as f64) / 3.;

    println!("Average of {arr:?} is {avg:.3}\nSum of {arr:?} is {sum}\nProduct of {arr:?} is {prod}\n");
    if arr[0] > arr[1] {
        println!("#1 is greater than #2");
    }
    else {
        println!("#2 is less than or equal to #1");
    }

    if arr[0] + arr[1] >= arr[2] {
        println!("#3 is hefty hefty hefty");
    }

    if (arr[0] & arr[1] & arr[2]) == 1 {
        println!("how odd\n");
    }
    else {
        println!("lucky duck\n");
    }

    for i in 0..5 {
        println!("{}", i + 1);
    }
}