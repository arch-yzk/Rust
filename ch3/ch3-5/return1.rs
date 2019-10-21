fn main()
{
    let z = add(10, 5);
    println!("10 + 5 = {}", z);
}

fn add(x: i32, y: i32) -> i32
{
    x + y
    //Rustでは戻り値は;をつけない
}