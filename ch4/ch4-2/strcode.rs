fn main()
{
    let letters = "abcde";
    println!("文字列の大きさ = {}", letters.len());
    println!("UTF-8 = {:?}", letters.as_bytes());

    let kanji = "Ohm社";
    println!("文字列の大きさ = {}", kanji.len());
    println!("UTF-8 = {:?}", kanji.as_bytes());
}