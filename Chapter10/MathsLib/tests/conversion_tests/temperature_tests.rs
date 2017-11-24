extern crate mathslib;=

mod temperature_test
{   
    use super::mathslib::conversions::temperature;
    
    #[test]
    fn test_kelvin_to_celcius_pass()
    {
        let calc = temperature::kelvin_to_celcius(14.5);
        assert_eq!(calc.0, true);
    }
    
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_kelvin_to_celcius_fail()
    {
        let calc = temperature::kelvin_to_celcius(-4f32);
        assert_eq!(calc.0,true);
    }
}