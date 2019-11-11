fn main()
{
    let mut msg = String::new();
    msg.push_str("Hello");
    msg.push(' ');
    msg.push_str("world.");

    //変数の値を表示
    println!("msg = {}", msg);
}