fn main()
{
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let first: &[i32] = &array[0..5];
    let second = &array[5..10];

    //スライスが参照するデータを表示
    println!("first = {:?}", first);
    println!("second = {:?}", second);
}