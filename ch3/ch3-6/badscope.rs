fn main()
{
    let x = 10;

    if true
    {
        let y = 20;
    }
    
    println!("x = {}\ny = {}", x, y);
    //yはスコープの範囲外なのでコンパイルできない
}