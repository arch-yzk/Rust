struct Cube
{
    width: i32,
    height: i32,
    depth: i32,
}

//立方体の実装
impl Cube
{
    fn get_volume(&self) -> i32
    {
        self.width * self.height * self.depth
    }

    fn get_area(&self) -> i32
    {
        self.width * self.height * 2 + 
        self.height * self.depth * 2 + 
        self.depth * self.width * 2
    }
}

fn main()
{
    let cube = Cube{width: 10, height: 20, depth: 30};

    println!("辺 = ({}, {}, {})", cube.width, cube.height, cube.depth);
    println!("容積 = {}", cube.get_volume());
    println!("表面積 = {}", cube.get_area());
}