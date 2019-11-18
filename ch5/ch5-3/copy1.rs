fn main()
{
    let x = 10;
    let y = 30;

    incr(x, y);
    println!("main:x = {},main:y = {}", x, y);
}

fn incr(mut x: i32, mut y: i32)
{
    x += 1;
    y += 1;
    println!("incr:x = {}, incr:y = {}", x, y);
}