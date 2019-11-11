fn main()
{
    //マクロによるベクタの初期化
    let mut vec: Vec<i32> = vec![1, 3, 5, 7, 9];

    //各要素の値を2倍にする
    for i in 0..5
    {
        vec[i] = vec[i] * 2;
    }

    //各要素を表示
    println!("vec = {:?}", vec);
}