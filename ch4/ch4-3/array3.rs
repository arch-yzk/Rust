fn main()
{
    const N: usize = 10;
    let mut array = [0; N];

    for i in 0..N
    {
        array[i] = 10 * i;
    }

    for i in 0..N
    {
        println!("array[{}] = {}", i, array[i]);
    }
}

