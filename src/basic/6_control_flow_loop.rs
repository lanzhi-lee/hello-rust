fn main() {
  // rust 提供了三种循环 while /for / loop

  // loop 循环
  let mut loop_count = 0;
  // 'outer: 是这个 loop 循环的标签
  'outer: loop {
    println!("loop_count = {}", loop_count);

    let mut remain = 10;
    loop {
      println!("remain = {}", remain);

      // 这一句中断内层循环
      if remain == 9 {
        break;
      }

      // 这一句中断外层循环 outer
      if loop_count == 2 {
        break 'outer;
      }
      remain -= 1;
    }

    loop_count += 1;
  }

  println!("end loop_count is {}", loop_count);

  // 有点像 while true
  // 但是 loop 在 break 时可以返回值，例如
  // 文档里讲 在某些服务出错时，可以使用loop循环进行重试
  let mut loop_return_count = 0;
  let loop_return = loop {
    if loop_return_count == 10 {
      // break (1, 2, 43); // ✅
      // break [1, 2, 43]; // ✅
      // break 100; // ✅
      // break 1 + 1; // 还可以跟表达式
      // break "字符串";
      break false;
      // 看起来 break 后面可以跟任意的值
    }

    loop_return_count += 1;
  };
  println!("loop_return is {:?}", loop_return);
  // 以下的代码是不能通过编译的
  // can only break with a value inside `loop` or breakable block
  // let mut while_return_count = 0;
  // let while_return = while true {
  //   if loop_return_count == 3 {
  //     break 3;
  //   }
  //   while_return_count += 1;
  // }

  // while 循环 跟传统的没啥区别
  let mut while_count = 0;
  while while_count < 3 {
    println!("while_count is {}", while_count);
    while_count += 1;
  }

  // for 循环

  // 传统的写法是不可以的
  // for (let mut index = 0; index < 3; index += 1){
  //   println!("{}", index);
  // }

  // 1..4 是一个 Range
  // 是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列
  for for_number in 1..4 {
    println!("for_number is {}", for_number);
  }

  // for 循环仍然经常出现在集合遍历
  let for_arr = [1, 2, 3, 4, 5];
  // 有点像 es6 的 for of 循环
  for elem in for_arr.iter() {
    println!("the current value of for_arr is {}", elem);
  }

  // 如果需要数组的下标，也可以使用下面的方法
  for index in 0..for_arr.len() {
    println!("the [{}] value of for_arr is {}", index, for_arr[index]);
  }
}
