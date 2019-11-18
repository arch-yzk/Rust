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