fn main()
{
    let b: Box <i32> = Box::new(10); 
    /*「let b: Box<i32> =」までがBox型宣言
    「Box::new(10)」でようやく値を入れられる*/
    println!("b = {}", b);
}