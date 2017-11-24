trait MyTrait
{
    fn is_done(&self) -> bool;
    fn is_not_done(&self) -> bool { println!("Is not done"); true }
}

struct UseFirstTime;
impl MyTrait for UseFirstTime
{
    fn is_done(&self) -> bool
    {
        println!("UseFirstTime.is_done");
        true
    }
}


struct OverrideFirstTime;
impl MyTrait for OverrideFirstTime
{
    fn is_done(&self) -> bool
    {
        println!("OverrideFirstTime.is_done");
        true
    }
    
    fn is_not_done(&self) -> bool
    {
        println!("OverrideFirstTime.is_not_done");
        true
    }
}


fn main() {
    let uft = UseFirstTime {};
    let oft = OverrideFirstTime {};
    uft.is_done();
    oft.is_not_done();    
}
