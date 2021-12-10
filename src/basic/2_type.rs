fn main() {
  // 整型
  // 分为有符号和无符号两种，分别以 i / u 为前缀
  // 按照位数，常见的有 8 16 32 64 128 位
  // 默认类型为 i32

  // 看起来似乎只能支持以上几种位数
  // 类似于 u9 u24 都不支持
  // u256 提示类型不在 scope
  let num: u8 = 255;
  println!("num = {}", num);

  // 按照不同进制，还有一些写法
  // 数值间可以使用下划线分割数字，增强可读性

  // 十进制
  let dec = 98_2021;
  println!("dec = {}", dec);

  // 十六进制
  let hex = 0xff_ff;
  println!("hex = {}", hex);

  // 八进制
  let oct = 0o77;
  println!("oct = {}", oct);

  // 二级制
  let bin = 0b1111;
  println!("bin = {}", bin);

  // 单字节的字符
  let byte = b'A'; // 65
  let byte2 = b'a'; // 97
  println!("byte = {}", byte);
  println!("byte2 = {}", byte2);

  // 浮点型
  // 支持 f32 和 f64 两种，默认为 f64
  // 分别为单精度和双精度

  let fx = 0.1;
  let fy = 0.2;
  let fz = fx + fy;
  // 默认的双精度浮点型 一样存在 0.30000000000000004 的问题
  // 改成单精度的 f32 似乎没问题了
  println!("fz = {}", fz);

  // 基本运算符
  // 加 减 乘 除 取模 不再记录
  // + - * / %

  // 布尔值
  // 类型名叫 bool，字面量写作 true / false

  // 字符类型 注意不是字符串
  // 单个字符使用单引号，占用 4 个字节的空间
  let c1 = 'z';
  let c2 = 'ℤ';
  let c3 = '😻';
  let c4 = "xxx"; // 字符传使用单引号会抛错
  println!("c1 = {}, c2 = {}, c3 = {}", c1, c2, c3);
  println!("c4 = {}", c4);

  // 复合类型
  // 元组和数组

  // 元组
  // 定长，可以包含多种类型
  let tup: (i32, f64, u8, char) = (500, 6.4, 1, 'b');
  // 直接打印是不可以的
  // println!("tup = {}", tup);
  // 可以使用下面的两种占位符代替
  println!("tup = {:?}", tup); // 单行
  println!("tup = {:#?}", tup); // 多行

  // 从元组中取值
  // 1. 解构
  let (tup_x, tup_y, tup_z, tup_char) = tup;
  // 这里还发现一个有趣的事情，rust 的变量不让用小驼峰，必须 snake_case
  // 另外解构看起来必须把全部元素取出，否则会报错
  println!(
    "tup_x = {}, tup_y = {}, tup_z = {}, tup_char = {}",
    tup_x, tup_y, tup_z, tup_char
  );

  // 2. 从索引取出
  println!(
    "tup.0 = {}, tup.1 = {}, tup.2 = {}, tup.3 = {}",
    tup.0, tup.1, tup.2, tup.3
  );
  // 尝试取超出长度的值 会直接报错 例如 tup.4
  // tup[3] 这种方式也不支持，会提示 用 tup.3 替代

  // 数组
  // 定长，单一类型
  let arr1 = [1, 2, 3, 4, 5]; // 基本用法
  println!("arr1 = {:?}", arr1); // 一样需要使用 {:?} 或者 {:#?} 来指定占位符

  // 指定类型和长度
  let arr2: [u32; 6] = [1, 2, 3, 4, 5, 9]; // 必须同时将类型和长度写明，不然会报错，例如 [u32] 并不会自动推断出长度为 6
  println!("arr2 = {:?}", arr2); // 一样需要使用 {:?} 或者 {:#?} 来指定占位符

  // 指定初始化值和长度
  let arr3 = [3; 2]; // [3, 3]

  // 访问数组元素
  // 1. 从索引取出
  println!("arr3[0] = {}", arr3[0]);
  // println!("arr3[3] = {}", arr3[3]); // 访问超出长度的元素会报错
  // println!("arr3.1 = {}", arr3.1); // 元组的这种访问方式，对数组是不支持的，看来元组 => tup.index ，数组 arr[index]

  // 2. 解构访问，注意使用的是 方括号
  // let (arr3_0, arr3_1) = arr3; // 圆括号直接抛错
  let [arr3_0, arr3_1] = arr3;
  println!("arr3_0 = {}, arr3_1 = {}", arr3_0, arr3_1);

  // let [, , arr1_2, ,arr1_4] = arr1; // 尝试取出某几个元素，看起来是失败了
}
