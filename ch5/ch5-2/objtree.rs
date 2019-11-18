struct Rectangle
{
    name: String,
    width: i32,
    height: i32,
}

impl Rectangle
{
    fn new(w: i32, h: i32) -> Rectangle
    {
        Rectangle
        {
            name: "長方形".to_string(), width: w, height: h
        }
    }
}

fn main()
{
    let mut rectvec = Vec::new();
    rectvec.push(Rectangle::new(10, 20));
    rectvec.push(Rectangle::new(100, 50));
    rectvec.push(Rectangle::new(30, 90));

    for rect in &rectvec
    {
        println!("幅{}、高さ{}の{}を生成しました。", rect.width, rect.height, rect.name);
    }
}