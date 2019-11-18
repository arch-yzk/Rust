fn main()
{
    let name1 = String::from("Alice");
    let name2 = name1;//所有権の移動

    println!("私の名前は{}です", name2);
}