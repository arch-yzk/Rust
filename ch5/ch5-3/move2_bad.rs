fn main()
{
    let name = String::from("Alice");

    println!("関数呼び出し");
    introduce(name);
    println!("関数終了");

    println!("{}", name);
}

fn introduce(myname: String)
{
    println!("私の名前は{}です", myname);
}