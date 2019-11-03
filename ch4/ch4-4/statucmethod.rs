struct Cube
{
    width: i32,
    height: i32,
    depth: i32,
}

impl Cube
{
    //コンストラクタ
    fn new(width: i32, height: i32, depth: i32) -> Cube
    {
        Cube{width: width, height: height, depth: depth}
    }

    //立方体の面の数
    fn get_num_surfaces() -> i32
    {
        6
    }
}

fn main()
{
    let cube = Cube::new(10, 20, 50);

    println!("辺の長さ = ({}, {}, {})", cube.width, cube.height, cube.depth);
    println!("面の数 = {}", Cube::get_num_surfaces());
}