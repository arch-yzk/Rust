fn main()
{
    let x = 20;

    if x > 20
    {
        println!("x = {}", x);
        println!("xは20より大きい");
    }

    if (x + 30) >= 50
    {
        println!("x = {}", x);
        println!("xは50以上");
    }

    if true
    {
        println!("必ず実行される(if true)");
    }
}