fn main()
{
    let name = String::from("Alice");

    println!("関数を呼び出す");
    introduce(name);//所有権の移動
    println!("関数処理が終了しました");
}

fn introduce(myname: String)
{
    println!("私の名前は{}です", myname);
    //終了しても所有権は再移動しない
}