use std::fmt::Display;

fn main() {
    let context_data = String::from("System Context");
    let processor_data = String::from("Processor A");
    
    // 'a: 上下文，存活时间最长
    let context = Context { name: &context_data };
    
    {
        // 'b: 处理器，生命周期比 'a 短
        let processor = Processor { id: &processor_data };
        
        {
            // 'c: 临时请求，生命周期最短
            let request = "Request #1";
            
            // 这里我们调用复杂生命周期函数
            // 只有当 'b 能够“活得”比 'c 长 ('b: 'c)，且 'a 比 'b 长 ('a: 'b) 时，
            // 这种层级依赖才成立。
            // 虽然在这个具体函数调用中，Rust 常常能自动推导，但显式的约束能表达这种层级关系。
            let result = execute_operation(&context, &processor, request);
            println!("Result: {}", result);
        }
    }
}

struct Context<'a> {
    name: &'a str,
}

struct Processor<'b> {
    id: &'b str,
}

// 这个函数展示了复杂的生命周期依赖关系
// 我们使用 where 子句来明确这些约束
fn execute_operation<'a, 'b, 'c>(
    context: &'a Context<'a>,
    processor: &'b Processor<'b>,
    request: &'c str
) -> &'b str 
where 
    'a: 'b, // 约束：Context ('a) 必须活得比 Processor ('b) 长 (outlive)
    'b: 'c, // 约束：Processor ('b) 必须活得比 Request ('c) 长
{
    println!("Context [{}] is using Processor [{}] to handle '{}'", 
             context.name, processor.id, request);

    // 假设我们的逻辑是：处理器会处理这个请求，并返回处理器自己的 ID 作为凭证。
    // 因为返回的是 processor.id，它的生命周期是 'b。
    // 由于我们声明了 'a: 'b，如果我们需要返回 context 里的东西也是安全的（降级为 'b）。
    // 但是我们不能返回 request，因为它的生命周期 'c 可能比 'b 短。
    
    processor.id
}
