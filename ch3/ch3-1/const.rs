fn main()
{
    const PI: i32 = 3;
    let radius = 10;

    //円周の計算
    let cir = 2 * PI * radius;
    println!("円周 = {}", cir);

    //面積の計算
    let area = PI * radius * radius;
    println!("面積 = {}", area);
}