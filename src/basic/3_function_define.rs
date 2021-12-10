fn foo2() {
  println!("outer foo2");
}

fn main() {
  // 函数使用 fn 关键字进行声明
  // 格式为 fn 函数名 { 函数体 }

  // 看起来 函数可以定义在主函数内外前后的任意地方
  // 应该也有类似 JS 的函数提升的机制，确保定义的函数可被访问

  fn foo4() {
    println!("inner foo4");
  }

  foo1();
  foo2();
  foo3();
  foo4();

  fn foo3() {
    println!("inner foo3");
  }
}

fn foo1() {
  println!("outer foo1");
}
