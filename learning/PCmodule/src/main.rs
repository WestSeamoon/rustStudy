use std::collections::HashMap;              //标准库下的collections下的HashMap struct
//针对函数以外的惯例，直接指定到本身

//use std::fmt;                               //同名目录，指定到父级
//同名目录引用也可以用as以区分
//use std::io::Result as IoResult;
//use std::cmp::Ordering;

//使用嵌套清理大量的use语句,上面三条引用合并为一条
use std::{fmt::Result,io::Result as IoResult,cmp::Ordering};

//特殊情况的合并引用
//use std::io;
//use std::io::Write
use std::io::{self,Write};

fn f1() -> fmt::Result{}                    //红色报错是因为函数没有返回值

fn f2() ->IoResult{}


//使用通配符把路径中所有的公共条目都引入到作用域
use std::collections::*;                     //把std::collections下的所有公共条目都引入作用域

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
