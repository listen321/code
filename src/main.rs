// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let x: i32 = 1; // 未初始化，但被使用
//     let y: i32; // 未初始化，也未被使用
//     println!("x is equal to {}", x);
// }

// 完形填空，让代码编译
// fn main() {
//     let mut x = 1;
//     x += 2;
//
//     println!("x = {}", x);
// }

// 修复下面代码的错误并使用尽可能少的改变
// fn main() {
//     let x: i32 = 10;
//     let y: i32 = 5;
//     {
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     println!("x 的值是 {}, y 的值是 {}", x, y);
// }

// 修复错误
// fn main() {
//     let x = define_x();
//     println!("{}, world", x);
// }
//
// fn define_x() -> String {
//     let x = "hello".to_string();
//     return x;
// }
// fn main() {
//     let x = define_x();
//     println!("{:?}, world", x);
// }
//
// fn define_x() -> &'static str {
//     let x = "hello";
//     return x;
// }
//

// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }
//
//     assert_eq!(x, 5);
//
//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }

// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let x = x;
//     // x += 3;
//
//     let y = 4;
//     // 遮蔽
//     let y = "I can also be bound to text!";
//     println!("success")
// }
//
// #[allow(unused_variables)]
// fn main() {
//     let x = 1;
// }
//
// // compiler warning: unused variable: `x`
//

// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;
//
//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// fn main() {
//     let (x, y);
//     (x, ..) = (3, 4);
//     [.., y] = [1, 2];
//     // 填空，让代码工作
//     assert_eq!([x, y], [3, 2]);
// }

// 移除某个部分让代码工作
// fn main() {
//     let x: i32 = 5;
//     let mut y = 5;
//
//     y = x;
//
//     let z = 10; // 这里 z 的类型是?
// }

// 填空
// fn main() {
//     let v: u16 = 38_u8 as u16;
//     println!("v: {v}")
// }

// 修改 `assert_eq!` 让代码工作
// fn main() {
//     let x = 5;
//     assert_eq!("i32".to_string(), type_of(&x));
// }
//
// // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// 填空，让代码工作
// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 255);
// }

// 解决代码中的错误和 `panic`
// fn main() {
//     let v1 = 247_u8 + 8;
//     let v2 = i8::checked_add(119, 8).unwrap();
//     println!("{},{}", v1, v2);
// }
//

// 修改 `assert!` 让代码工作
// fn main() {
//     let x = 0b1111_1111;
//     println!("x: {x}");
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1597);
// }

// 将 ? 替换成你的答案
// fn main() {
//     let x = 1_000.000_1; //
//     let y = 0.12; //
//     let z = 0.01_f64; //
//     println!("{y}")
// }
//

// fn main() {
//     assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
//     assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.001);
// }
// fn main() {
//     let mut sum = 0;
//     for i in -3..3 {
//         sum += i
//     }
//
//     println!("{sum}");
//     assert!(sum == -3);
//
//     for c in 'a'..='z' {
//         println!("{}", c as u8);
//     }
// }

// 填空
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range { start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));
// }

// 填空，并解决错误
// fn main() {
//     // 整数加法
//     assert!(1u32 + 2 == 3);
//
//     // 整数减法
//     assert!(1i32 - 2 == -1);
//     assert!(1i8 - 2 == -1);
//
//     assert!(3 * 50 == 150);
//
//     assert!(9.6_f32 / 3.2_f32 == 3.0_f32); // error ! 修改它让代码工作
//
//     assert!(24 % 5 == 4);
//
//     // 逻辑与或非操作
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);
//
//     // 位操作
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }
//
// 修改2处 `assert_eq!` 让代码工作

// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1), 4);
//
//     let c2 = '中';
//     assert_eq!(size_of_val(&c2), 4);
//
//     println!("Success!")
// }
// 修改一行让代码正常打印
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// }
//
// fn print_char(c: char) {
//     println!("{}", c);
// }

// 使成功打印
// fn main() {
//     let _f: bool = false;
//
//     let t = false;
//     if !t {
//         println!("Success!")
//     }
// }

// fn main() {
//     let f = true;
//     let t = true && false;
//     assert_eq!(t, !f);
//
//     println!("Success!")
// }

// 让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();
//
//     let v = (2, 3);
//     assert_eq!(v, implicitly_ret_unit());
//
//     println!("Success!")
// }
//
// fn implicitly_ret_unit() -> (i32, i32) {
//     println!("I will return a ()");
//     (2, 3)
// }
//
// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }

// 让代码工作：修改 `assert!` 中的 `4`
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);
//
//     println!("Success!")
// }

// fn main() { let x = 5u32;
//
//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;
//
//         // 下面表达式的值将被赋给 `y`
//         x_cube + x_squared + x
//     };
//
//     let z = {
//         // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
//         2 * x;
//     };
//
//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }
// 使用两种方法让代码工作起来
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2;
//         x
//     };
//
//     assert_eq!(v, 3);
// }

// fn main() {
//     let v = {
//         let x = 3;
//         x
//     };
//
//     assert!(v == 3);
// }

// fn main() {
//     let s = sum(1, 2);
//     assert_eq!(s, 3);
// }
//
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);
//
//     assert_eq!(s, 3);
// }
//
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn main() {
//     print();
// }
//
// // 使用另一个类型来替代 i32
// fn print() -> () {
//     println!("hello,world");
// }

// 用两种方法求解
//  fn main() {
//      never_return();
//  }
//
//  fn never_return() -> ! {
//      panic!("i return nothing") // 实现这个函数，不要修改函数签名!
//  }

// fn main() {
//     println!("Success!");
// }
//
// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };
//
//     // 这里与其返回一个 None，不如使用发散函数替代
//     never_return_fn()
// }
//
// // 使用三种方法实现以下发散函数
// fn never_return_fn() -> ! {
//     loop {
//         // std::thread::sleep(std::time::Duration::from_secs(1))
//         // todo!()
//         unimplemented!()
//     }
// }

// fn main() {
//     // 填空
//     let b = false;
//
//     let _v = match b {
//         true => 1,
//         // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
//         false => {
//             println!("Success!");
//             panic!("we have no value for `false`, but we can panic")
//         }
//     };
//
//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     // 使用尽可能多的方法来通过编译
//     // let x = &String::from("hello, world");
//     let x = String::from("hello, world");
//     // let y = x.clone();
//     // let y = x;
//     let y = x.as_str();
//     println!("{},{}", x, y);
//     // println!("{}", y);
// }

// 不要修改 main 中的代码
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);
//
//     println!("{}", s2);
// }
//
// // 只能修改下面的代码!
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     return s;
// }

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
//
// // 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     // let _s = s.clone().into_bytes();
//     let _s = s.as_bytes();
//     s
// }

// 修复错误，不要删除任何代码行
// fn main() {
//     let s = String::from("hello, world");
//
//     print_str(&s);
//
//     println!("{}", s);
// }
//
// fn print_str(s: &String) {
//     println!("{}", s)
// }

// 不要使用 clone，使用 copy 的方式替代
// fn main() {
//     let x = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }

// fn main() {
//     let s = String::from("hello, ");
//
//     // 只修改下面这行代码 !
//     let mut s1 = s;
//
//     s1.push_str("world")
// }

// fn main() {
//     let x = Box::new(5);
//
//     let mut y = Box::new(3); // 完成该行代码，不要修改其它行！
//
//     *y = 4;
//
//     assert_eq!(*x, 5);
// }

// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }
//
//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };
//
//     // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
//     // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
//     let Person { name, ref age } = person;
//
//     println!("The person's age is {}", age);
//
//     println!("The person's name is {}", name);
//
//     // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
//     //println!("The person struct is {:?}", person);
//
//     // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
//     println!("The person's age from person struct is {}", person.age);
// }

// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//
//     let _s = t.0;
//
//     // 仅修改下面这行代码，且不要使用 `_s`
//     println!("{:?}", t.1);
// }

// fn main() {
//     let t = (String::from("hello"), String::from("world"));
//
//     // 填空，不要修改其它代码
//     let (s1, s2) = &t;
//
//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }

// fn main() {
//     let x = 5;
//     // 填写空白处
//     let p = &x;
//
//     println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
// }

// fn main() {
//     let x = 5;
//     let y = &x;
//
//     // 只能修改以下行
//     assert_eq!(5, *y);
// }

// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//
//     borrow_object(&s)
// }
//
// fn borrow_object(s: &String) {}

// 修复错误
// fn main() {
//     let mut s = String::from("hello, ");
//
//     push_str(&mut s)
// }
//
// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

// fn main() {
//     let mut s = String::from("hello, ");
//
//     // 填写空白处，让代码工作
//     let p = &mut s;
//
//     p.push_str("world");
// }

//

// fn main() {
//     let c = '中';
//
//     let r1 = &c;
//     // 填写空白处，但是不要修改其它行的代码
//     let ref r2 = c;
//
//     assert_eq!(*r1, *r2);
//
//     // 判断两个内存地址的字符串是否相等
//     assert_eq!(get_addr(r1), get_addr(r2));
// }
//
// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }

// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = s.clone();
//     let r2 = &mut s;
//
//     println!("{}, {}", r1, r2);
// }

// fn main() {
//     // 通过修改下面一行代码来修复错误
//     let mut s = String::from("hello, ");
//
//     borrow_object(&mut s)
// }
//
// fn borrow_object(s: &mut String) {}

// 下面的代码没有任何错误
// fn main() {
//     let mut s = String::from("hello, ");
//
//     borrow_object(&s);
//
//     s.push_str("world");
// }
//
// fn borrow_object(s: &String) {}
//

// 注释掉一行代码让它工作
// fn main() {
//     let mut s = String::from("hello, ");
//
//     let r1 = &mut s;
//     r1.push_str("world");
//     let r2 = &mut s;
//     r2.push_str("!");
//
//     // println!("{}", r1);
// }

// fn main() {
//     let mut s = String::from("hello, ");
//
//     let r1 = &mut s;
//     let r2 = &mut s;
//
//     // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
//     // 你不能同时使用 r1 和 r2
//     println!("{}, {}", r1, r2);
// }

// 修复错误，不要新增代码行
// fn main() {
//     let s: &str = "hello, world";
// }

// 使用至少两种方法来修复错误
// fn main() {
//     let s: Box<&str> = "hello, world".into();
//     greetings(*s)
// }
//
// fn greetings(s: &str) {
//     println!("{}", s)
// }

// 填空
// fn main() {
//     let mut s = String::new();
//     s.push_str("hello, world");
//     s.push('!');
//
//     assert_eq!(s, "hello, world!");
// }

// 修复所有错误，并且不要新增代码行
// fn main() {
//     let mut s = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += &"!".to_string();
//
//     println!("{}", s)
// }

// 填空
// fn main() {
//     let s = String::from("I like dogs");
//     // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
//     let s1 = s.replace("dogs", "cats");
//
//     assert_eq!(s1, "I like cats")
// }

// 修复所有错误，不要删除任何一行代码
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1.clone() + &s2;
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s1);
// }

// 使用至少两种方法来修复错误
// fn main() {
//     // let s = "hello, world".to_string();
//     let s = String::from("hello, world");
//     greetings(s)
// }
//
// fn greetings(s: String) {
//     println!("{}", s)
// }

// 使用两种方法来解决错误，不要新增代码行
// fn main() {
//     // let s = "hello, world".to_string();
//     let s = "hello, world";
//     // let s1: &str = &s;
//     // let s1: String = s;
//     let s1: &str = s;
// }
// fn main() {
//     // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
//     // 填空以输出 "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
//
//     // 也可以使用 Unicode 形式的转义字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
//
//     println!(
//         "Unicode character {} (U+211D) is called {}",
//         unicode_codepoint, character_name
//     );
//
//     // 还能使用 \ 来连接多行字符串
//     let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }

/* 填空并修复所有错误 */
// fn main() {
//     let raw_str = "Escapes don't work here: \x3F \u{211D}";
//     // 修改上面的行让代码工作
//     assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
//
//     // 如果你希望在字符串中使用双引号，可以使用以下形式
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);
//
//     // 如果希望在字符串中使用 # 号，可以如下使用：
//     let delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);
//
//     // 填空
//     let long_delimiter = "Hello, \"##\"";
//     assert_eq!(long_delimiter, "Hello, \"##\"")
// }

// use std::str;
//
// fn main() {
//     // 注意，这并不是 `&str` 类型了！
//     let bytestring: &[u8; 21] = b"this is a byte string";
//
//     // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
//     println!("A byte string: {:?}", bytestring);
//
//     // 字节数组也可以使用转义
//     let escaped = b"\x52\x75\x73\x74 as bytes";
//     // ...但是不支持 unicode 转义
//     // let escaped = b"\u{211D} is not allowed";
//     println!("Some escaped bytes: {:?}", escaped);
//
//     // raw string
//     let raw_bytestring = br"\u{211D} is not escaped here";
//     println!("{:?}", raw_bytestring);
//
//     // 将字节数组转成 `str` 类型可能会失败
//     if let Ok(my_str) = str::from_utf8(raw_bytestring) {
//         println!("And the same as text: '{}'", my_str);
//     }
//
//     let _quotes = br#"You can also use "fancier" formatting, \
//                     like with normal raw strings"#;
//
//     // 字节数组可以不是 UTF-8 格式
//     let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS
//
//     // 但是它们未必能转换成 `str` 类型
//     match str::from_utf8(shift_jis) {
//         Ok(my_str) => println!("Conversion successful: '{}'", my_str),
//         Err(e) => println!("Conversion failed: {:?}", e),
//     };
// }

// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
//     assert_eq!(h, "h");
//
//     let h1 = &s1[3..6]; // 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
//     assert_eq!(h1, "中");
// }

// fn main() {
//     // 填空，打印出 "你好，世界" 中的每一个字符
//     for c in "你好，世界".chars() {
//         println!("{}", c)
//     }
// }

// use utf8_slice;
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";
//
//     let rocket = utf8_slice::slice(s, 4, 5);
//     // 结果是 "🚀"
// }

// fn main() {
//     // 使用合适的类型填空
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//
//     // 修改以下代码，让它顺利运行
//     assert!(arr.len() == 5);
// }
// fn main() {
//     // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
//     let arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c'];
//
//     // 填空
//     // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
//     // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
//     assert!(std::mem::size_of_val(&arr) == 12);
// }
//
// fn main() {
//     // 填空
//     let list: [i32; 100] = [1; 100];
//
//     assert!(list[0] == 1);
//     assert!(list.len() == 100);
// }

// fn main() {
//     // 修复错误
//     let _arr = [1, 2, 3];
// }

// fn main() {
//     let arr = ['a', 'b', 'c'];
//
//     let ele = arr[0]; // 只修改此行来让代码工作
//
//     assert!(ele == 'a');
// }

// 修复代码中的错误
// fn main() {
//     let names = [String::from("Sunfei"), "Sunface".to_string()];
//
//     // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
//     let name0 = names.get(0).unwrap();
//
//     // 但是下标索引就存在越界的风险了
//     let _name1 = &names[1];
// }

// 修复代码中的错误，不要新增代码行!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2];
//
//     let s2: &str = "hello, world";
// }

// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];
//
//     let slice = &arr[..2];
//
//     // 修改数字 `8` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
//     assert!(std::mem::size_of_val(&slice) == 16);
// }

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // 填空让代码工作起来
//     let slice: &[i32] = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);
// }

// fn main() {
//     let s = String::from("hello");
//
//     let slice1 = &s[0..2];
//     // 填空，不要再使用 0..2
//     let slice2 = &s[..2];
//
//     assert_eq!(slice1, slice2);
// }

// fn main() {
//     let s = "你好，世界";
//     // 修改以下代码行，让代码工作起来
//     let slice = &s[0..3];
//     assert!(slice == "你");
// }

// 修复所有错误
// fn main() {
//     let mut s = String::from("hello world");
//
//     // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
//     // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
//     let ch = first_character(&s);
//
//     println!("the first character is: {}", ch);
//
//     s.clear();
// }
// fn first_character(s: &str) -> &str {
//     &s[..1]
// }

// fn main() {
//     let _t0: (u8, i16) = (0, -1);
//     // 元组的成员还可以是一个元组
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     // 填空让代码工作
//     let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
// }

// 修改合适的地方，让代码工作
// fn main() {
//     let t = ("i", "am", "sunface");
//     assert_eq!(t.2, "sunface");
// }

// 修复代码错误
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
//     println!("too long tuple: {:?}", too_long_tuple);
// }

// fn main() {
//     let tup = (1, 6.4, "hello");
//
//     // 填空
//     let (x, z, y) = tup;
//
//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);
// }

// fn main() {
//     let (x, y, z);
//
//     // 填空
//     (y, z, x) = (1, 2, 3);
//
//     assert_eq!(x, 3);
//     assert_eq!(y, 1);
//     assert_eq!(z, 2);
// }

// fn main() {
//     // 填空，需要稍微计算下
//     let (x, y) = sum_multiply((2, 3));
//
//     assert_eq!(x, 5);
//     assert_eq!(y, 6);
// }
//
// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//     (nums.0 + nums.1, nums.0 * nums.1)
// }

// fix the error
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String,
// }
// fn main() {
//     let age = 30;
//     let p = Person {
//         name: String::from("sunface"),
//         age,
//         hobby: "".to_string(),
//     };
//     println!("{}", p.age)
// }

// struct Unit;
// trait SomeTrait {
//     // ...定义一些行为
// }
//
// // 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// // 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
// impl SomeTrait for Unit {}
// fn main() {
//     let u = Unit;
//     do_something_with_unit(u);
// }
//
// // 填空，让代码工作
// fn do_something_with_unit(u: Unit) {}

// 填空并修复错误
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(0, 127, 255);
//     check_color(v);
// }
//
// fn check_color(p: Point) {
//     let Point(x, _, _) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(p.2, 255);
// }

// 填空并修复错误，不要增加或移除代码行
// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {
//     let age = 18;
//     let mut p = Person {
//         name: String::from("sunface"),
//         age,
//     };
//
//     // how can you believe sunface is only 18?
//     p.age = 30;
//
//     // 填空
//     p.name = String::from("sunfei");
// }

// 填空
// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {}
//
// fn build_person(name: String, age: u8) -> Person {
//     Person { age, name }
// }

// 填空，让代码工作
// struct User { active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let u1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("sunface"),
//         active: true,
//         sign_in_count: 1,
//     };
//
//     let u2 = set_email(u1);
// }
//
// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }

// 填空，让代码工作
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
//         height: 50,
//     };
//
//     dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr
//
//     println!("{:?}", rect1); // 打印 debug 信息到标准输出 stdout
// }

// fn main() { #[derive(Debug)] struct Person { name: String,
//         age: Box<u8>,
//     }
//
//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };
//
//     // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
//     // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
//     let Person { name, ref age } = person;
//
//     println!("The person's age is {}", age);
//
//     println!("The person's name is {}", name);
//
//     // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
//     //println!("The person struct is {:?}", person);
//
//     // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
//     println!("The person's age from person struct is {}", person.age);
// }

// 修复错误
// #[derive(Debug)] struct File {
//     name: String,
//     data: String,
// }
// fn main() {
//     let f = File {
//         name: String::from("readme.md"),
//         data: "Rust By Practice".to_string(),
//     };
//
//     let _name = f.name;
//
//     // 只能修改这一行
//     println!("{}", f.data);
// }

// 修复错误
// enum Number {
//     Zero,
//     One,
//     Two,
// }
//
// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }
//
// // C语言风格的枚举定义
// enum Number2 {
//     Zero = 0,
//     One = 1,
//     Two = 2,
// }
//
// fn main() {
//     // 通过 `as` 可以将枚举值强转为整数类型
//     assert_eq!(Number::One as u8, Number1::One as u8);
//     assert_eq!(Number1::One as u8, Number2::One as u8);
// }

// 填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msg1 = Message::Move { x: 1, y: 2 }; // 使用x = 1, y = 2 来初始化
//     let msg2 = Message::Write("hello".to_string()); // 使用 "hello, world!" 来初始化
// }

// 仅填空并修复错误
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msg = Message::Move { x: 1, y: 1 };
//
//     if let Message::Move { x: a, y: b } = msg {
//         assert_eq!(a, b);
//     } else {
//         panic!("不要让这行代码运行！");
//     }
// }

// 填空，并修复错误
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msgs: [Message; 3] = [
//         Message::Quit,
//         Message::Move { x: 1, y: 3 },
//         Message::ChangeColor(255, 255, 0),
//     ];
//
//     for msg in msgs {
//         show_message(msg)
//     }
// }
//
// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }

// 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//
//     if let Some(n) = six {
//         println!("{}", n);
//         return;
//     }
//
//     panic!("不要让这行代码运行！");
// }
//
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// 填空，让代码运行
// use crate::List::*;
//
// enum List {
//     // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
//     Cons(u32, Box<List>),
//     // Nil: 链表中的最后一个节点，用于说明链表的结束
//     Nil,
// }
//
// // 为枚举实现一些方法
// impl List {
//     // 创建空的链表
//     fn new() -> List {
//         // 因为没有节点，所以直接返回 Nil 节点
//         // 枚举成员 Nil 的类型是 List
//         Nil
//     }
//
//     // 在老的链表前面新增一个节点，并返回新的链表
//     fn prepend(self, elem: u32) -> List {
//         Cons(elem, Box::new(self))
//     }
//
//     // 返回链表的长度
//     fn len(&self) -> u32 {
//         match *self {
//             // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
//             Cons(_, ref tail) => 1 + tail.len(),
//             // 空链表的长度为 0
//             Nil => 0,
//         }
//     }
//
//     // 返回链表的字符串表现形式，用于打印输出
//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 // 递归生成字符串
//                 format!("{}, {}", head, tail.stringify())
//             }
//             Nil => {
//                 format!("Nil")
//             }
//         }
//     }
// }
//
// fn main() {
//     // 创建一个新的链表(也是空的)
//     let mut list = List::new();
//
//     // 添加一些元素
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);
//
//     // 打印列表的当前状态
//     println!("链表的长度是: {}", list.len());
//     println!("{}", list.stringify());
// }

// 填空
// fn main() {
//     let n = 5;
//
//     if n < 0 {
//         println!("{} is negative", n);
//     } else if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// }

// 修复错误
// fn main() {
//     let n = 5;
//
//     let big_n = if n < 10 && n > -10 {
//         println!(" 数字太小，先增加 10 倍再说");
//
//         10 * n
//     } else {
//         println!("数字太大，我们得让它减半");
//
//         n / 2
//     };
//
//     println!("{} -> {}", n, big_n);
// }

// fn main() {
//     for n in 1..=100 {
//         // 修改此行，让代码工作
//         if n == 101 {
//             panic!("NEVER LET THIS RUN")
//         }
//     }
// }

// 修复错误，不要新增或删除代码行
// fn main() {
//     let names = [String::from("liming"), String::from("hanmeimei")];
//     for name in &names {
//         // do something with name...
//     }
//
//     println!("{:?}", names);
//
//     let numbers = [1, 2, 3];
//     // numbers中的元素实现了 Copy，因此无需转移所有权
//     for n in numbers {
//         // do something with name...
//     }
//
//     println!("{:?}", numbers);
// }

// fn main() {
//     let a = [4, 3, 2, 1];
//
//     // 通过索引和值的方式迭代数组 `a`
//     for (i, v) in a {
//         println!("第{}个元素是{}", i + 1, v);
//     }
// }

// fn main() {
//     let a = [4, 3, 2, 1];
//
//     // iterate the indexing and value in 'a'
//     for (i, v) in a.iter().enumerate() {
//         println!("The {}th element is {}", i + 1, v);
//     }
// }

// 填空，让最后一行的  println! 工作 !
// fn main() {
//     // 一个计数值
//     let mut n = 1;
//
//     // 当条件为真时，不停的循环
//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }
//
//         n += 1;
//     }
//
//     println!("n 的值是 {}, 循环结束", n);
// }

// 填空，不要修改其它代码
// fn main() {
//     let mut n = 0;
//     for i in 0..=100 {
//         if n == 66 {
//             break;
//         }
//         n += 1;
//     }
//
//     assert_eq!(n, 66);
// }

// 填空，不要修改其它代码
// fn main() {
//     let mut count = 0u32;
//
//     println!("Let's count until infinity!");
//
//     // 无限循环
//     loop {
//         count += 1;
//
//         if count == 3 {
//             println!("three");
//
//             // 跳过当此循环的剩余代码
//             continue;
//         }
//
//         println!("{}", count);
//
//         if count == 5 {
//             println!("OK, that's enough");
//
//             break;
//         }
//     }
//
//     assert_eq!(count, 5);
// }

// 填空
// fn main() {
//     let mut counter = 0;
//
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break 20;
//         }
//     };
//
//     assert_eq!(result, 20);
// }

// 填空
// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 // 这只会跳出 inner1 循环
//                 break 'inner1; // 这里使用 `break` 也是一样的
//             }
//             count += 2;
//         }
//
//         count += 5;
//
//         'inner2: loop {
//             if count >= 30 {
//                 break 'outer;
//             }
//
//             continue 'outer;
//         }
//     }
//
//     assert!(count == 30)
// }

// 填空
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }
//
// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South | Direction::North => {
//             // 在这里匹配 South 或 North
//             println!("South or North");
//         }
//         _ => println!("West"),
//     };
// }

// fn main() {
//     let boolean = true;
//
//     // 使用 match 表达式填空，并满足以下条件
//     //
//     // boolean = true => binary = 1
//     // boolean = false => binary = 0
//     let binary = match boolean {
//         true => 1,
//         false => 0,
//     };
//
//     assert_eq!(binary, 1);
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move { x: 1, y: 3 },
//         Message::ChangeColor(255, 255, 0),
//     ];
//
//     for msg in msgs {
//         show_message(msg)
//     }
// }
//
// fn show_message(msg: Message) {
//     match msg {
//         Message::Move { x: a, y: b } => {
//             // match  Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         }
//         Message::ChangeColor(_, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         _ => println!("no data in these variants"),
//     }
// }

// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
//
//     // 使用 `matches` 填空
//     for ab in alphabets {
//         assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
//     }
// }

// enum MyEnum {
//     Foo,
//     Bar,
// }
//
// fn main() {
//     let mut count = 0;
//
//     let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
//     for e in v {
//         if matches!(e, MyEnum::Foo) {
//             // 修复错误，只能修改本行代码
//             count += 1;
//         }
//     }
//
//     assert_eq!(count, 2);
// }

// fn main() {
//     let o = Some(7);
//
//     if let Some(i) = o {
//         println!("This is a really long string and `{:?}`", i);
//     }
//     // 移除整个 `match` 语句块，使用 `if let` 替代
//     // match o {
//     //     Some(i) => {
//     //         println!("This is a really long string and `{:?}`", i);
//     //     }
//     //     _ => {}
//     // };
// }

// 填空
// enum Foo {
//     Bar(u8),
// }
//
// fn main() {
//     let a = Foo::Bar(1);
//
//     if let Foo::Bar(i) = a {
//         println!("foobar 持有的值是: {}", i);
//     }
// }

// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32),
// }
//
// fn main() {
//     let a = Foo::Qux(10);
//     match a {
//         Foo::Bar => {
//             println!("match foo::bar")
//         }
//         Foo::Baz => {
//             println!("match foo::baz")
//         }
//         __ => {
//             println!("match others")
//         }
//     }
//     // 移除以下代码，使用 `match` 代替
//     // if let Foo::Bar = a {
//     //     println!("match foo::bar")
//     // } else if let Foo::Baz = a {
//     //     println!("match foo::baz")
//     // } else {
//     //     println!("match others")
//     // }
// }

// fn main() {
//     let age = Some(30);
//     if let Some(age) = age {
//         // create a new variable with the same name as previous `age`
//         assert_eq!(age, 30);
//     } // the new variable `age` goes out of scope here
//
//     match age {
//         // match can also introduce a new shadowed variable
//         Some(age) => println!("age is a new variable, it's value is {}", age),
//         _ => (),
//     }
// }

// fn main() {
//     match_number(5)
// }
// fn match_number(n: i32) {
//     match n {
//         // 匹配一个单独的值
//         1 => println!("One!"),
//         // 使用 `|` 填空，不要使用 `..` 或 `..=`
//         2..=5 => println!("match 2 -> 5"),
//         // 匹配一个闭区间的数值序列
//         6..=10 => {
//             println!("match 6 -> 10")
//         }
//         _ => {
//             println!("match 11 -> +infinite")
//         }
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }
//
// fn main() {
//     // 填空，让 p 匹配第二个分支
//     let p = Point { x: 1, y: 20 };
//
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // 第二个分支
//         Point {
//             x: 0..=5,
//             y: y @ (10 | 20 | 30),
//         } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }

// 修复错误
// enum Message {
//     Hello { id: i32 },
// }
//
// fn main() {
//     let msg = Message::Hello { id: 5 };
//
//     match msg {
//         Message::Hello { id: id @ 3..=7 } => println!("id 值的范围在 [3, 7] 之间: {}", id),
//         Message::Hello {
//             id: newid @ (10 | 11 | 12),
//         } => {
//             println!("id 值的范围在 [10, 12] 之间: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }

// 填空让代码工作，必须使用 `split`
// fn main() {
//     let num = Some(8);
//     let split = 5;
//     match num {
//         Some(x) if x < split => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }
// }

// 填空，让代码工作
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
//
//     match numbers {
//         (first, .., last) => {
//             assert_eq!(first, 2);
//             assert_eq!(last, 2048);
//         }
//     }
// }

// 修复错误，尽量少地修改代码
// 不要移除任何代码行
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;
//
//     match r {
//         value => value.push_str(" world!"),
//     }
//     println!("{v}")
// }

// struct Point {
//     x: f64,
//     y: f64,
// }
//
// // `Point` 的关联函数都放在下面的 `impl` 语句块中
// impl Point {
//     // 关联函数的使用方法跟构造器非常类似
//     fn origin() -> Point {
//         Point { x: 0.0, y: 0.0 }
//     }
//
//     // 另外一个关联函数，有两个参数
//     fn new(x: f64, y: f64) -> Point {
//         Point { x: x, y: y }
//     }
// }
//
// struct Rectangle {
//     p1: Point,
//     p2: Point,
// }
//
// impl Rectangle {
//     // 这是一个方法
//     // `&self` 是 `self: &Self` 的语法糖
//     // `Self` 是当前调用对象的类型，对于本例来说 `Self` = `Rectangle`
//     fn area(&self) -> f64 {
//         // 使用点操作符可以访问 `self` 中的结构体字段
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;
//
//         // `abs` 是一个 `f64` 类型的方法，会返回调用者的绝对值
//         ((x1 - x2) * (y1 - y2)).abs()
//     }
//
//     fn perimeter(&self) -> f64 {
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;
//
//         2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
//     }
//
//     // 该方法要求调用者是可变的，`&mut self` 是 `self: &mut Self` 的语法糖
//     fn translate(&mut self, x: f64, y: f64) {
//         self.p1.x += x;
//         self.p2.x += x;
//
//         self.p1.y += y;
//         self.p2.y += y;
//     }
// }
//
// // `Pair` 持有两个分配在堆上的整数
// struct Pair(Box<i32>, Box<i32>);
//
// impl Pair {
//     // 该方法会拿走调用者的所有权
//     // `self` 是 `self: Self` 的语法糖
//     fn destroy(self) {
//         let Pair(first, second) = self;
//
//         println!("Destroying Pair({}, {})", first, second);
//
//         // `first` 和 `second` 在这里超出作用域并被释放
//     }
// }
//
// fn main() {
//     let rectangle = Rectangle {
//         // 关联函数的调用不是通过点操作符，而是使用 `::`
//         p1: Point::origin(),
//         p2: Point::new(3.0, 4.0),
//     };
//
//     // 方法才是通过点操作符调用
//     // 注意，这里的方法需要的是 `&self` 但是我们并没有使用 `(&rectangle).perimeter()` 来调用，原因在于：
//     // 编译器会帮我们自动取引用
//     //  `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
//     println!("Rectangle perimeter: {}", rectangle.perimeter());
//     println!("Rectangle area: {}", rectangle.area());
//
//     let mut square = Rectangle {
//         p1: Point::origin(),
//         p2: Point::new(1.0, 1.0),
//     };
//
//     // 错误！`rectangle` 是不可变的，但是这个方法要求一个可变的对象
//     // rectangle.translate(1.0, 0.0);
//     // TODO ^ 试着反注释此行，看看会发生什么
//
//     // 可以！可变对象可以调用可变的方法
//     square.translate(1.0, 1.0);
//
//     let pair = Pair(Box::new(1), Box::new(2));
//
//     pair.destroy();
//
//     // Error! 上一个 `destroy` 调用拿走了 `pair` 的所有权
//     // pair.destroy();
//     // TODO ^ 试着反注释此行
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     // 完成 area 方法，返回矩形 Rectangle 的面积
//     fn area(self) -> u32 {
//         self.width * self.height
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     assert_eq!(rect1.area(), 1500);
// }

// 只填空，不要删除任何代码行!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     pub fn show_state(&self) {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light = TrafficLight {
//         color: "red".to_owned(),
//     };
//     // 不要拿走 `light` 的所有权
//     light.show_state();
//     // 否则下面代码会报错
//     println!("{:?}", light);
// }

// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // 使用 `Self` 填空
//     pub fn show_state(self: &self) {
//         println!("the current state is {}", &self.color);
//     }
//
//     // 填空，不要使用 `Self` 或其变体
//     pub fn change_state(&mut self) {
//         self.color = "green".to_string()
//     }
// }
// fn main() {}

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // 1. 实现下面的关联函数 `new`,
//     // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
//     // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
//     pub fn new(color: String) -> Self {
//         Self { color: color }
//     }
//
//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }
//
// fn main() {
//     let light = TrafficLight::new("red".to_string());
//     assert_eq!(light.get_state(), "red");
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// // 使用多个 `impl` 语句块重写下面的代码
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// fn main() {}

// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }
//
// // 为 TrafficLightColor 实现所需的方法
// impl TrafficLightColor {
//     fn color(&self) -> String {
//         match *self {
//             TrafficLightColor::Red => "red".to_string(),
//             TrafficLightColor::Yellow => "yellow".to_string(),
//             TrafficLightColor::Green => "green".to_string(),
//         }
//     }
// }
//
// fn main() {
//     let c = TrafficLightColor::Yellow;
//
//     assert_eq!(c.color(), "yellow");
//
//     println!("{:?}", c);
// }
