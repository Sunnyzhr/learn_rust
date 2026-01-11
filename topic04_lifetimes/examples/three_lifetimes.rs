fn main() {
    // 'a: text 拥有最长的生命周期，它是我们想要返回的数据源
    let text = String::from("The quick brown fox jumps over the lazy dog");
    let result;
    
    {
        // 'b: delimiter 是一个临时的分隔符，它的生命周期很短
        let delimiter = String::from("fox"); 
        
        // 'c: annotation 只是一个临时的日志消息，它的生命周期也很短
        let annotation = String::from("Looking for split point"); 
        
        // 调用函数
        // 注意：虽然 delimiter 和 annotation 马上就要“死”了，
        // 但这不应该影响我们返回 text 的一部分。
        // 如果我们强制所有参数都是 'a，这行代码就会报错，因为 delimiter 和 annotation 活得不够长。
        result = extract_prefix(&text, &delimiter, &annotation);
        
    } // delimiter 和 annotation 在这里消亡

    // result 依然活着！因为它借用的是 text ('a)，而不是那些短命的变量。
    println!("Result: '{}'", result);
}

// 这个函数使用了三个不同的生命周期
// 'a: 主要文本的生命周期 (返回值依赖于它)
// 'b: 分隔符的生命周期 (仅用于查找，不返回)
// 'c: 注释消息的生命周期 (仅用于打印，不返回)
fn extract_prefix<'a, 'b, 'c>(
    text: &'a str, 
    delimiter: &'b str, 
    annotation: &'c str
) -> &'a str {
    println!("Log [{}]: Splitting text by '{}'", annotation, delimiter);
    
    if let Some(index) = text.find(delimiter) {
        // 返回 text 的一个切片，它的生命周期自然也是 'a
        return &text[..index];
    }
    
    text
}
