fn main()
{
    func();
}

fn func()
{
    let s1 = String::from("Alice");
    let s2 = String::from("Bob");
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    drop(s1);

    let s1 = String::from("Chris");
    println!("s1 = {}", s1);
}