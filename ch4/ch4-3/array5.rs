fn main()
{
    let names: [&str; 4] = ["Alice", "Bob", "Chris", "David"];

    for i in 0..4
    {
        println!("names[{:?}] = {:?}", i, names[i]);
    }
}