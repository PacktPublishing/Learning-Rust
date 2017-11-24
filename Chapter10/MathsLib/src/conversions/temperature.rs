//pub mod temperature
//{
    pub fn farenheit_to_celcius(f: f32) -> f32
    {
        (f - 32f32) * 5f32/9f32
    }
    
    pub fn celcius_to_farenheit(c: f32) -> f32
    {
        (c * (9f32/5f32)) + 32f32
    }
    
    pub fn celcius_to_kelvin(c: f32) -> (bool, f32)
    {
        return (true, c + 273.15);
    }
    
    pub fn kelvin_to_celcius(k: f32) -> (bool, f32)
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
    
    pub fn farenheit_to_kelvin(f: f32) -> f32
    {
        (f + 459.67) * 5f32 / 9f32
    }
    
    pub fn kelvin_to_farenheit(k: f32) -> f32
    {
        (k * (9f32 / 5f32)) - 459.67
    }
//}

