fn main()
{
    let x = 20;

    if x > 10 && x < 30 
    {
        println!("x = {}", x);
        println!("xは10より大きい、かつ30より小さい");
    }

    if x <= 10 || x >= 30
    {
        println!("x = {}", x);
        println!("xは10以下、もしくは30以上");
    }

    if !(x < 0)
    {
        println!("x = {}", x);
        println!("x非負の値");
    }
}