fn main()
{
    let a:f64 = 1000.0;
    let b:f64 = 33.0;
    let c = 2.5;

    let x = a / b;
    let y = b / a;
    let z = a / c;

    println!("{} / {} = {}", a, b, x);
    println!("{} / {} = {}", b, a, y);
    println!("{} / {} = {}", a, c, z);
}