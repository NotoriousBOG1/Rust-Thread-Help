use std::process;
use std::io;
use std::io::prelude::*;

fn main() {
    dummy_practice();
    let stdin = io::stdin();
    for _line in stdin.lock().lines() {
        println!("-----------------------------------------------");
        dummy_practice()

    }
}

fn dummy_practice() {
    let var_int = &123456;
    let var_string = "DefaultString";
    let mut array_char = Vec::with_capacity(128);
    array_char.resize(128, "");

    let ptr2int = format!("{:p}", var_int);
    let ptr2ptr = &ptr2int;
    let _ptr2ptr2 = &ptr2ptr;
    println!("Process ID: {}", process::id());
    println!("\n");
    println!("varInt ({:p}) = {}", var_int, var_int);
    println!("var_string ({:p}) = {}", var_string, var_string);
    println!("\n");
    println!("ptr2int ({:p}) = {:p}", &ptr2int, var_int);
    println!("ptr2ptr ({:p}) = {:p}", &ptr2ptr, &ptr2int);
    println!("\n");
    println!("Press ENTER to print again");
}
