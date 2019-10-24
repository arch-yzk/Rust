static GLOBAL: i32 = 99;

fn main()
{
    let local: i32 = 10;
    let mut ptr: *const i32 = &local;

    unsafe
    {
        println!("*ptr = {}", *ptr);
    }

    println!("ptr = {:?}", ptr);

    ptr = &GLOBAL;

    unsafe
    {
        println!("*ptr = {}", *ptr);
    }

    println!("ptr = {:?}", ptr);
}