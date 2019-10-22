fn main()
{
    let var = factorial(5);
    println!("5の階乗 = {}", var);
}

fn factorial(n: i32) -> i32
{
    if n == 1
    {
        return 1;
    }
    else
    {
        factorial(n - 1) * n
    }
}