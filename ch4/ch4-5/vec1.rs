fn main()
{
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(3);
    vec.push(5);

    //Vectaの情報を表示
    println!("vecの大きさ = {}", vec.len());
    println!("vecの各要素 = {:?}", vec);
    println!("vec[1] = {:?}", vec.get(1));
    println!("vec[1] = {}", vec.get(1).unwrap());

    //2つめの要素を削除
    vec.remove(1);
    println!("remove後のvec = {:?}", vec);
}