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
