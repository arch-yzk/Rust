static mut PI: f64 = 3.14;

fn get_cir(radius: f64) -> f64
{
    unsafe //排他制御が保証できないため
    {
        PI * radius * 2.0
    }
}

fn get_area(radius: f64) -> f64
{
    unsafe
    {
        PI * radius * radius
    }
}

fn main()
{
    let radius = 10.0;

    unsafe
    {
        PI = 3.0;
    }

    let cir = get_cir(radius);
    let area = get_area(radius);

    println!("円周 = {}", cir);
    println!("面積 = {}", area);
}