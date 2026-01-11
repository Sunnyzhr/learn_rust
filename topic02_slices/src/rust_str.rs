/// 1. str (核心基元：字符串切片类型)
/// 本质：动态大小类型 (DST - Dynamically Sized Type)。
/// 特点：
/// - 不能直接存放在栈上（因为编译器不知道它多大）。
/// - 必须通过指针（如 &str, Box<str>）来使用。
/// - 代表内存中一段有效的 UTF-8 字节序列。
///
/// 内存布局：
/// [ str ] (不定长，直接是一串字节，必须依附于某个位置)
///  ^
///  | 无法独立存在于栈上，必须隐藏在指针后面
#[test]
fn test_type_str() {
    // ❌ 错误示范：无法编译
    // let s: str = "hello"; // Error: the size for values of type `str` cannot be known at compilation time
    
    // ✅ 正确用法：必须在指针后面
    let _s: &str = "hello";
    // 或者在 Box 中
    let _b: Box<str> = "hello".into();
}

/// 2. String (核心实体：可增长字符串)
/// 本质：拥有所有权的堆分配字符串 (Owned Heap Allocation)。
/// 特点：
/// - 实际上是一个封装了 Vec<u8> 的结构体。
/// - 包含三个字段：ptr (指针), len (长度), capacity (容量)。
/// - 负责内存的申请和释放。
///
/// 内存布局 (Stack -> Heap)：
/// [ ptr ] -> [ h, e, l, l, o ] (Heap)
/// [ len ]    (5)
/// [ cap ]    (5)
#[test]
fn test_type_string() {
    let s: String = String::from("hello");
    assert_eq!(s.len(), 5);
    // s 离开作用域时，堆内存会被自动释放
}

/// 3. &str (核心形式：字符串切片引用)
/// 本质：对 str 的不可变借用 (Immutable Borrow)。
/// 特点：
/// - "胖指针" (Fat Pointer)：包含 ptr (指向数据起始) + len (数据长度)。
/// - 最常用的字符串参数类型，因为它可以指向 String，也可以指向字面量。
///
/// 内存布局 (Stack -> Heap/Static)：
/// [ ptr ] -> [ h, e ] (可以是 Heap 的一部分，也可以是 Static)
/// [ len ]    (2)
#[test]
fn test_type_ref_str() {
    let s1: String = String::from("hello");
    let literal: &'static str = "world";

    // 它可以指向 String 的一部分
    let slice1: &str = &s1[0..2]; // "he"
    // 也可以指向二进制数据段的字面量
    let slice2: &str = literal;   // "world"

    assert_eq!(slice1, "he");
    assert_eq!(slice2, "world");
}

/// 4. &String (容器的引用)
/// 本质：对 String 结构体的不可变借用。
/// 特点：
/// - 指向 String 的元数据（栈上的 ptr/len/cap）。
/// - ⚠️ 不推荐作为函数参数。应使用 &str。
/// - Rust 有 "Deref Coercion" (解引用强制转换)，&String 会自动转为 &str。
///
/// 内存布局 (Stack -> Stack -> Heap)：
/// [ ptr ] -> [ ptr ] -> [ h, e, l, l, o ] (Heap)
///            [ len ]
///            [ cap ]
/// (指向 String 结构体的指针)
#[test]
fn test_type_ref_string() {
    let s = String::from("hello");
    
    // 显式类型为 &String
    let ref_s: &String = &s;
    
    // 接受 &str 的函数也能接受 &String (自动转换)
    fn takes_str(input: &str) -> usize { input.len() }
    
    // 这里发生了隐式转换: &String -> &str
    // Rust 编译器自动执行了 Deref: &String -> &str
    assert_eq!(takes_str(ref_s), 5); 
}

/// 5. mut str (不可能的变量)
/// 本质：试图创建一个可变的 DST 变量。
/// 结论：❌ 语法上无法直接声明 `let mut s: str`。
/// 只有在极少数通过 Box<str> 解引用时可能遇到概念上的 mut str，但实际上还是通过引用操作。
///
/// 内存布局：
/// (不存在。因为无法在栈上分配未知大小的可变空间)
#[test]
fn test_type_mut_str_binding() {
    // ❌ 绝对无法编译
    // let mut s: str = "hello"; 
    
    // 勉强相关的概念：Box<str> 的可变性
    let mut b: Box<str> = "hello".into();
    // 我们可以获取 &mut str 来修改它
    b.make_ascii_uppercase(); 
    assert_eq!(&*b, "HELLO");
}

/// 6. mut String (可变的实体)
/// 本质：一个允许修改的 String 变量。
/// 特点：
/// - 只有声明为 mut，才能修改它（push, pop, clear）。
///
/// 内存布局 (Stack -> Heap)：
/// [ ptr ] -> [ h, e, l, l, o, _, _, _ ] (Heap, 预留了空间)
/// [ len ]    (5)
/// [ cap ]    (8)
/// (完全同 String，只是编译器允许修改这个结构体的内容)
#[test]
fn test_type_mut_string() {
    // 必须加 mut 才能修改
    let mut s = String::from("hello");
    s.push_str(" world"); // ✅ 合法
    assert_eq!(s, "hello world");
}

/// 7. &mut str (可变的切片引用)
/// 本质：对 str 的可变借用。
/// 特点：
/// - 允许原地修改数据（如大小写转换）。
/// - ⚠️ 严禁修改长度（没有 capacity，无法扩容）。
/// - 只能操作 UTF-8 字节，必须保证修改后仍是有效 UTF-8。
///
/// 内存布局 (Stack -> Heap)：
/// [ ptr ] -> [ h, e, l, l, o ] (Heap)
/// [ len ]    (5)
/// (同 &str，只是拥有了对指向数据的修改权)
#[test]
fn test_type_mut_ref_str() {
    let mut s = String::from("hello");
    
    // 获取可变切片
    let slice: &mut str = &mut s[..];
    
    // ✅ 合法：原地修改内容 (ASCII 变换不改变字节长度)
    slice.make_ascii_uppercase();
    
    // ❌ 非法：slice.push('!'); // &mut str 没有 push 方法
    
    assert_eq!(s, "HELLO");
}

/// 8. &mut String (可变的容器引用)
/// 本质：对 String 结构体的可变借用。
/// 特点：
/// - 拥有最大权限：既能改内容，也能改长度（扩容/缩容）。
/// - 它是 String 修改操作（如 push_str）所需的 `self` 类型。
///
/// 内存布局 (Stack -> Stack -> Heap)：
/// [ ptr ] -> [ ptr ] -> [ h, e, l, l, o ] (Heap)
///            [ len ]
///            [ cap ]
/// (指向 String 结构体的可变指针，可以修改 len 和 cap，甚至重新分配 Heap)
#[test]
fn test_type_mut_ref_string() {
    let mut s = String::from("hello");
    
    // 定义一个接受 &mut String 的函数
    fn add_exclamation(buf: &mut String) {
        buf.push('!'); // ✅ 可以改变长度
    }
    
    add_exclamation(&mut s);
    assert_eq!(s, "hello!");
}

/// 9. String 所有权转移 (Move)
/// 本质：将栈上的元数据 (ptr, len, cap) 复制给新变量，同时原变量失效。
/// 关键：堆上的数据没有发生拷贝！
///
/// 内存布局变化：
/// Before Move:
/// Stack A [ptr] -> Heap [h, e, l, l, o]
///
/// After Move (传参后):
/// Stack A [ptr] (失效)    Heap [h, e, l, l, o] (位置不变)
/// Stack B [ptr] --------^ (新主人)
#[test]
fn test_string_move() {
    let s1 = String::from("hello");
    let ptr_before = s1.as_ptr(); // 记录堆内存地址
    
    // 定义一个接收 String 所有权的函数
    fn take_ownership(s: String) -> String {
        // s 在这里是新主人
        s // 返回它，把所有权移出来，防止被 drop
    }
    
    // 发生 Move：s1 的所有权移交给函数的参数 s，然后又移交给 s2
    // 此时 s1 已经失效，不能再访问
    let s2 = take_ownership(s1); 
    
    let ptr_after = s2.as_ptr();
    
    // 验证：堆内存地址没有变！说明没有发生深拷贝。
    assert_eq!(ptr_before, ptr_after);
}

/// 10. String 深拷贝 (Clone)
/// 本质：显式调用 clone()，在堆上分配新的内存，复制所有数据。
/// 结果：产生两份完全独立的数据，原变量依然有效。
///
/// 内存布局变化：
/// Before Clone:
/// Stack A [ptr] -> Heap A [h, e, l, l, o]
///
/// After Clone:
/// Stack A [ptr] -> Heap A [h, e, l, l, o] (保持原样)
/// Stack B [ptr] -> Heap B [h, e, l, l, o] (全新内存)
#[test]
fn test_string_clone() {
    let s1 = String::from("hello");
    let ptr1 = s1.as_ptr();
    
    // 显式拷贝：s2 拥有全新的堆内存
    // 只有这样，才能在不转移所有权的情况下，让 s2 拥有数据
    let s2 = s1.clone(); 
    let ptr2 = s2.as_ptr();
    
    // 验证1：s1 依然有效 (没有被 Move)
    assert_eq!(s1, "hello");
    
    // 验证2：堆内存地址不同！说明发生了深拷贝
    assert_ne!(ptr1, ptr2);
}

/// 11. &mut String 的双重可变性辨析
/// 辨析：是指针本身能变？还是指向的内容能变？
///
/// 结论：
/// 1. `let r: &mut String` -> r 本身不可变(不能改指别人)，但可以通过 r 修改 String。
/// 2. `let mut r: &mut String` -> r 本身可变(能改指别人)，也可以通过 r 修改 String。
#[test]
fn test_mut_ref_semantics() {
    let mut s1 = String::from("s1");
    let mut s2 = String::from("s2");

    // Case 1: 内容可变 (Mutable Referent)
    // r1 是不可变绑定 (没有 mut)，但它是一个可变引用 (&mut String)
    let r1: &mut String = &mut s1;
    r1.push_str("_modified"); // ✅ 合法：可以修改指向的内容
    // r1 = &mut s2;          // ❌ 非法：不能修改 r1 本身让它指向 s2

    // Case 2: 指针本身可变 (Mutable Binding)
    // r2 是可变绑定 (有 mut)，且是一个可变引用
    let mut r2: &mut String = &mut s1;
    r2.push_str("_again");    // ✅ 合法：可以修改内容
    
    r2 = &mut s2;             // ✅ 合法：可以修改 r2 本身，让它指向 s2
    r2.push_str("_modified"); // 现在修改的是 s2
    
    assert_eq!(s1, "s1_modified_again");
    assert_eq!(s2, "s2_modified");

    // 修复后的 zhr 例子：使用 unsafe 的 as_bytes_mut 来修改切片内容
    let mut z1 = String::from("zhr");
    
    // 获取可变引用，并安全地修改 ASCII 字符
    // 注意：Rust 不允许直接 z2[0] = b'Z'，必须先获取字节切片
    let z2: &mut str = &mut z1[..]; 
    unsafe {
        let bytes = z2.as_bytes_mut();
        bytes[0] = b'Z';
        bytes[1] = b'H';
        bytes[2] = b'R';
    }
    
    assert_eq!(z1, "ZHR");
}
