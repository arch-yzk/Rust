fn main()
{
    let mut counter = 0;
    for i in 0..10
    {
        counter = incr(counter);
        println!("loop i = {}:counter = {}", i, counter);
    }
}

fn incr(cnt: i32) -> i32
{
    if cnt >= 8
    {
        println!("カウンタの値をリセットします");
        return 1;
        //returnを使用すれば;ありの宣言文でOK
    }
    else
    {
        println!("カウンタの値を1増やします");
        cnt + 1
        //カウンタを増やして「戻り値」を指定しているので;ナシ
    }
}