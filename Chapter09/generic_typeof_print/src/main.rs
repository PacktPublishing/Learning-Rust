#![feature(core_intrinsics)]
fn display_type<T>(_: &T)
{
    let typename = unsafe {std::intrinsics::type_name::<T>()};
    println!("{}", typename);
}

fn main()
{
    display_type(&3.14f32);
    display_type(&1i32);
    display_type(&1.555);
    display_type(&(vec!(1,3,5)));
}
