extern crate mathslib;
use mathslib::conversions::temperature::*;

fn main() 
{
    let testval = celcius_to_farenheit(100f32);
    println!("100C = {}F", testval);
    
    let testtuple = celcius_to_kelvin(14.5f32);
    println!("14.5C = {}K", testtuple.1);
}
