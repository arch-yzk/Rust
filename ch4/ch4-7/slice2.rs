fn main()
{
    let msg = String::from("My name is Alice.");
    let slice1 = &msg[0..7];
    let slice2 = &msg[11..msg.len()];

    //スライスが参照するデータを表示
    println!("slice1 = {:?}", slice1);
    println!("slice2 = {:?}", slice2);
}