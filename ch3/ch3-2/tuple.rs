fn main()
{
    let p = (10, 25);
    println!("(x, y) = ({}, {})", p.0, p.1);

    let q = (5, 10, 30);
    println!("(x, y, z) = ({}, {}, {})" , q.0, q.1, q.2);

    let s = (80, 90, 85, true);

    let (math, english, verbal, result) = s;
    println!("(数学、英語、国語、合否) = ({}, {}, {}, {})", math, english, verbal, result);

    let (_, _, _, result2) = s;
    println!("合否 = {}", result2);
}