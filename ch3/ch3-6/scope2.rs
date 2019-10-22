fn main()
{
    let mut x = 0;

    for i in 0..3
    {
        for j in 0..3
        {
            let y = i * 10 + j;
            x += 1;
            println!("y = {}", y);
        }
        
        let j = 'a'; //上のjはすでにスコープの範囲外
        println!("j = {}", j);
    }

    println!("x = {}", x);
}