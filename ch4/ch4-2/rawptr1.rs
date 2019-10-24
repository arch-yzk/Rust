fn main()
{
    let x: i32 = 1;
    let ptr: *const i32 = &x;//xに対してのポインタ変数

    //ptrが指すアドレスを値を表示
    unsafe
    {
        println!("*ptr = {}", *ptr);
    }

    println!("ptrの値 = {:?}", ptr);
}