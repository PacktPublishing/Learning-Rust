fn kelvin_to_celcius(k: f32) -> (bool, f32)
    {
        if k < 0f32
        {
        return (false, k);
        }
        else
        {
        return (true, k - 273.15);
        }
    }

fn main() 
{
    let mut calc = kelvin_to_celcius(14.5);
    match calc.0
    {
        true => println!("14.5K = {}C", calc.1),
        _ => println!("equation was invalid"),
    }
    
    calc = kelvin_to_celcius(-4f32);
    match calc.0
    {
        true => println!("-4K = {}C", calc.1),
        _ => println!("invalid K"),
    }
}