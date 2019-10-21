fn main()
{
    add(10, 5);
}

fn add(x: i32, y: i32)
{
    let z = x + y;
    println!("add({}, {}) = {}", x, y, z);
}