fn foo1(x: u8) {
  println!("the value of x is : {}", x);
}

// 提示 redefine 不是正确的重载写法
// fn foo1(x: u8, y: u16) {
//   println!("x = {}, y = {}", x, y);
// }

// 区分语句和表达式
// 语句（Statements）是执行一些操作但不返回值的指令
// 表达式（Expressions）计算并产生一个值
fn sentence() {
  // 1. 这种在 C 语言中支持的赋值方式，是不允许的
  // 原因是 赋值语句不会返回值 （实际上会隐式返回一个空元组）
  // let mut x = 5;
  // let y = x = 6;
  // println!("x = {}, y = {:?}", x, y)

  // 2. 宏调用是语句，也不返回值，一样的会存在一个 空元组
  let prt = println!("macro call");
  println!("macro call return is {:?}", prt);

  // 3. 代码块是语句，不返回值
  let code_block = {
    let _x = 1;
    // return _x; // 即使尝试 return，也不可以，会提示 expected `()`, found integer
  }; // 代码块结尾也需要有分号
  println!("code_block is {:?}", code_block);
}

// 函数声明时，返回值类型必须显式指定！
fn returns() -> u8 {
  // return 129; // 常规写法
  // 129 // 不加分号的值，也能返回，震惊了
  // 'b' // char 类型也支持这种骚操作 其他的类型不再一一尝试了，估计也能支持
  // 129; // 上面的骚操作是不可以加分号的，会提示 expected `u8`, found `()`

  // 文档里说 上面的这种骚操作还可以支持表达式，比如
  // fn demo(x: u8) -> u8 {
  //   x + 1
  // }
  // 确实整的有点愣，咱应该是不会这么玩的

  let x = 1;
  let y = 254;
  return x + y; // 返回表达式的结果
                // 按理说 整型的默认类型是 i32，但是上面的语句正常返回了，猜测内部做了类型转换
                // 尝试改成 255 + 1，会提示溢出，attempt to compute `1_u8 + u8::MAX`, which would overflow
}

fn main() {
  foo1(128);
  // foo1(256); // 必须准确传入类型，256是 u16 了，会报错

  // foo1(128, 256); // 这种方式似乎不是正确的重载写法

  sentence();

  // const return_val: u8 = returns(); // 不可以用const 声明接收函数返回值的变量
  let return_val: u8 = returns(); // 接收函数返回值的变量也必须显式指定类型
  println!("return_val = {}", return_val);
}
