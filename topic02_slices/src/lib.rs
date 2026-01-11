// Exercise: First Word
//
// Goal: Write a function that takes a string slice and returns a slice pointing to the first word.
// If no space is found, return the whole string.

pub fn first_word(s: &str) -> &str {
    // TODO: Implement this function
    // 1. Convert string to bytes array: s.as_bytes()
    // 2. Iterate with iter().enumerate()
    // 3. If byte is b' ', return slice &s[0..i]
    // 4. Return &s[..] if no space

    let bytes = s.as_bytes();
    // 柏拉图：我们在洞穴（循环）之外，先确立一个默认的“理型”——即整个字符串本身。
    // 为了能在洞穴中改变这种认知，我们需要赋予它“变化”（mut）的属性
    let mut ret = s;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 当我们在洞穴中发现了分界（空格），我们便修正我们的认知。
            ret = &s[0..i];
            // 既然真理已现，便无需继续徘徊。
            break;
        }
    }

    ret
}

// 柏拉图：若你想修改洞穴中的影像，你必须拥有“可变引用”的权杖 (&mut)。
// 但请记住，修改是危险的，特别是对于 UTF-8 字符串。
// 这是一个通过可变引用修改字符串内容的例子。
//
// 问：为何区分 String 和 str？
// 答：String 是实体（Owner），它拥有内存，可以扩容（push）。
//     str 是形式（View），它只是对数据的投影，大小固定，不可增长。
//     我们向 s (String) 中追加内容，内容来源于 word (str)。
pub fn concat_string(s1: &mut String, s2: String) -> String {
    // 1. 向 s1 追加 " + "
    // 问：为什么用 push_str(" + ") 而不是 push(' + ')？
    // 答：push 接受 char（单个字符，如 'a'）。' + ' 包含三个字符（空格+空格），是一个字符串切片，因此必须用 push_str。
    //
    // 问：为什么 push_str(" + ") 比 push_str(&String::from(" + ")) 好？
    // 答：
    // 1. push_str(" + "): 
    //    - 字面量 " + " 的类型本身就是 &'static str (引用)。
    //    - 只有 1 次拷贝：从只读数据段(rodata) -> s1 的堆内存。
    // 2. push_str(&String::from(" + ")): 
    //    a. 在堆上分配内存创建临时 String。
    //    b. 从只读数据段拷贝到临时 String。
    //    c. 从临时 String 拷贝到 s1。
    //    d. 销毁临时 String，释放堆内存。
    //    结论：后者多了 1 次分配、1 次释放、1 次多余拷贝。
    s1.push_str(" + ");

    // 2. 向 s1 追加 s2
    s1.push_str(&s2);

    // 3. 题目要求返回一个新的 String，其内容与 s1 相同。
    // 因为 s1 是一个借用 (&mut)，我们不能夺取它的所有权，只能“克隆”一份它的影像。
    // 返回类型：String (新的所有者)
    // 动作：
    // 1. clone(): 在堆上深拷贝数据 (Deep Copy)。
    // 2. return: 将新的 String 结构体(所有权) 移动 (Move) 给调用者。
    s1.clone()
}

pub mod rust_str;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("HelloWorld"), "HelloWorld");
        assert_eq!(first_word("Hello World"), "Hello");
        assert_eq!(first_word("Rust"), "Rust");
        assert_eq!(first_word("Good Morning Rust"), "Good");
    }

    #[test]
    fn test_append_word() {
        // 1. 创造实相时，必须声明它是可变的 (mut)
        let mut s1 = String::from("s1");
        let s2 = String::from("s2");
        let s3 = concat_string(&mut s1, s2);
        assert_eq!(s1, "s1 + s2");
        assert_eq!(s3, "s1 + s2");
        assert_eq!(s1, s3);
    }
}
