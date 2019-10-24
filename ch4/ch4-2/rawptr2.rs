fn main()
{
    let mut x: i32 = 1;
    let ptr: *mut i32 = &mut x;

    unsafe
    {
        println!("変更前:アドレス = {:?}, xの値 = {}", ptr, *ptr);

        *ptr = 99;
        println!("変更後:アドレス = {:?}, xの値 = {}", ptr, *ptr);
    }
}