fn main() {
  // rust 的变量默认不可变
  // let x = 5; // 不可变
  let mut x = 7; // 可变
  println!("x = {}", x);
  x = 6;
  println!("x = {}", x);

  // 可变值重复使用let 下面的会覆盖上面的，可执行
  // 这种操作被称为 “隐藏”
  let x = 8;
  println!("x = {}", x);

  // 例如 可以直接使用原变量来接收字符的长度
  // str -> number
  let spaces = "   ";
  let spaces = spaces.len();
  // 直接隐藏变量，会被编译器警告 需要给变量增加 _ 前缀

  // 常量一直不可变
  const CONSTANT: u32 = 1000;
  // const CONSTANT: u32 = 100 * 100; // 常量再次声明，编译不会通过
  println!("CONSTANT = {}", CONSTANT);
}
