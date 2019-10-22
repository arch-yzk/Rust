fn main()
{
    let mut x: i32 = 10;

    func(x);

    x = x / 2;
    println!("main: x = {}", x);
}

fn func(mut x: i32)
{
    x = x * 2;

    println!("func: x = {}", x);
}