//  cargo run --example dangling

fn main() {
    println!("code below is dangerous!")
    // let r;
    // {
    //     let s = String::from("hello");
    //     // 试图返回一个借用，其生命周期超过了 s
    //     r = dangerous_ref(&s); 
    // } // s 在这里被销毁
    
    // println!("{}", r); // 错误：这里 r 依然活着，但 s 已经死了
}

// 假如 Rust 允许返回值的生命周期比入参长...
// 这里 'b 比 'a 长（这是不可能通过编译的，只是为了演示）
fn dangerous_ref<'a, 'b>(s: &'a str) -> &'b str 
where 'a: 'b // 意思是 'a 必须活得比 'b 长 (Rust 默认规则)
{
    s
}

