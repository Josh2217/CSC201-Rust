extern crate scanner_rust;
use scanner_rust::Scanner;
use scanner_rust::generic_array::typenum::U64;

pub fn run(){
    println!("Assignment 3\n");

    println!("Josh Johnson\n");
    let mut scan: Scanner<_, U64> = Scanner::scan_path2("ref/txt/input1.txt").unwrap();
    let mut arr: [f64; 1000] = [0.; 1000];
    for i in 0..1000 {
        //this is ugly but it works
        arr[i] = scan.next_f64().unwrap().unwrap();
    }
    let mut sum: f64 = 0.;
    for n in arr {
        sum += n;
    }
    let μ = sum / arr.len() as f64;
    println!("Mean: {μ:.3}");
    sum = 0.;
    for n in arr {
        sum += (n - μ) * (n - μ) / (arr.len() - 1) as f64;
    }
    let σ = sum.sqrt();
    println!("Standard Deviation: {σ:.3}");
    //println!("{arr:?}");
    let mut count = 0;
    for i in 0..arr.len() {
        if i % 10 == 0 {print!("\n");}
        let append: char;
        if arr[i] > μ{
            append = '>';
            count += 1;
        }
        else if arr[i] == μ {
            append = '=';
        }
        else{
            append = '<';
        }
        print!("{:<5.2} {append}  ", arr[i])

    }
    println!("\n\nThere are {count} values greater than the mean: {μ:.3}");
    println!("There are {} values less than or equal to the mean: {μ:.3}", 1000 - count);
}