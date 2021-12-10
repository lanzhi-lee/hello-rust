fn main() {
  let score = 60;

  // if (score >= 90) { // 这种习惯的写法可以通过编译，但会被警告
  // help: remove these parentheses
  if score >= 90 {
    println!("很优秀");
  } else if score >= 90 {
    println!("优秀");
  } else if score >= 60 {
    println!("及格");
  } else {
    println!("不及格");
  }

  // 每个分支必须包含 花括号
  // 下面的写法 编译不能通过
  // if (score >= 60) println!("及格");

  // 条件表达式中不会自动进行类型转换 .... 感觉有点
  // if(score) { println!("有成绩"); }

  // 似乎也不存在空值的概念 score != null 之类的也是无效的
  if score >= 0 {
    println!("有成绩");
  }

  // rust 不支持三元运算符！！！
  // let level = score >= 60 ? "及格" : "不及格";
  // 万一真有类似的想法，可以以下面这种代替，（最终还是屈服在了这种奇怪的返回值办法之下）
  let level = if score >= 60 { "及格" } else { "不及格" };
  println!("level = {}", level);

  // 上面的写法是因为 代码块是可以返回值的，像这样
  let code_block_val = {
    // do something
    128 // 这行不能加分号
  };
  println!("code_block_val = {}", code_block_val);

  // rust 不支持传统的 switch case 语句 !!!
  // 但是提供了 match 语句，还没看到，以后再说
}
