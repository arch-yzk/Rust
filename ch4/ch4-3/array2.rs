fn main()
{
    let array: [f64;4] = [1.3, 23.2, 0.0, -55.2];

    for i in 0..4
    {
        println!("array[{}] = {}", i, array[i]);
    }
}