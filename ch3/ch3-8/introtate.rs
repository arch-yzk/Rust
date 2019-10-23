fn main()
{
    let x:i8 = 0b01001110;
    let y:i8 = x.rotate_left(3);

    println!("y = {:b}", y);
}