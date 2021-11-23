/*
Travis Nevins
simple commandline app

Current app takes kg weight and prints out what the weight will be on other planets
TODO:
    add math examples
    recursion examples
    functional examples 
    structs example
    pattern matching example
    file reading/editing example 
*/

use std::io;

fn main() {
    println!("Rust project! Bevy!");
    println!("Running weight on planets of the our solar system ");
    planet_weight();
}

//get user input used for other methods 
fn get_input() -> String {
    let mut input = String::new() ;
    io::stdin().read_line(&mut input).unwrap();
    return input;
}

//--------------------------------------
// Name: get_mars_weight
// args: floating point weight given fomr user and planet gravity
// return floating point weight in kg on Mars
fn get_new_weight(weight: f32 , planet_g: f32) -> f32 {
    (weight / 9.81) * planet_g  
}

//there are some implict returns by not using semicolons or the keyword
//moon 1.622
//saturn 11.08
//mar 3.77 
//mercury 3.59
//venus 8.87
//Uranus  10.67
//Neptune 14.07
//Jupiter 25.95
fn planet_weight() {
    println!("Enter weight kg please:");
    let input = get_input();
    
    let weight: f32 = input.trim().parse().unwrap();
    let moon = get_new_weight(weight, 1.622);
    let mars = get_new_weight(weight, 3.77);
    let venus = get_new_weight(weight, 8.87);
    let mercury = get_new_weight(weight, 3.59);
    let jupiter  = get_new_weight(weight, 25.95);
    let uranus = get_new_weight(weight, 10.67);
    let neptune = get_new_weight(weight, 17.07);
    let saturn = get_new_weight(weight, 11.08);
    println!("Your weight on Earth: {}", weight);
    println!("Your weight on moon: {}", moon);
    println!("Your weight on mercury: {}", mercury);
    println!("Your weight on venus: {}", venus);
    println!("Your weight on mars: {}", mars);
    println!("Your weight on jupiter: {}", jupiter);
    println!("Your weight on saturn: {}", saturn);
    println!("Your weight on uranus: {}", uranus);
    println!("Your weight on neptune: {}", neptune);
}