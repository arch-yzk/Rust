fn main()
{
    let mut x = 10;

    if x < 0
    {
        let y = x + 1;
        x = x + 1;
        println!("y = {}", y);
    }
    else
    {
        let z = x - 1;
        x = x - 1;
        println!("z = {}", z);
    }
    
    println!("x = {}", x);
}