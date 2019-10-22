fn main()
{
    let x = 10;
    let y = 30;

    let point = get_point(x, y);
    println!("点 = ({}, {})", point.0, point.1);
}

fn get_point(x: i32, y: i32) -> (i32, i32)
{
    (x, y)
}