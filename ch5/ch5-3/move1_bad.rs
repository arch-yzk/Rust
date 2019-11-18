fn main()
{
    let name1 = String::from("Alice");
    let name2 = name1;

    println!("私の名前は{}です", name2);

    //この行はコンパイルエラー
    println!("{}", name1);
}