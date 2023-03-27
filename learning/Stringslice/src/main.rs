fn main() {
    let mut s = String::from("Hello world");
    let worldindex = first_world(&s);

    //如果把字符串清0，worldindex依然为5，两者不相关
    //s.clear();                        //这里不会报错
    println!("The first world 的索引为:{}",worldindex);

    let s1 = String::from("Hello world");
    let world = first_worldqp(&s1);

    //s1.clear();       强关联，不能改，会报错        这是一个可变的引用，first_worldqp已经作为不可变引用了
    println!("The first world 为:{}",world);
    
    //字符串切片
    let hello = &s[0..5];       //从0开始不包括5，从0开始0可以不写
    let world = &s[6..11];      //5为空格，11如果为字符串长度，可以写作s.len()，也可以不写
    let whole = &s[..];         //切片指向整个字符串

    //定义一个切片
    let s2 = "Hello World";     //切片不可变

    //其他类型切片
    let a = [1,2,3,4,5];
    let slice = &a[1..3];




    //将切片作为函数参数
    let my_string = String::from("Hello world");
    let world_index = first_worldstr(&my_string[..]);

    let my_string_literal = "Hello world";
    let world_index = first_worldstr(my_string_literal);



}


//普通返回字符串下标
fn first_world(s:&String) -> usize{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{        //空格的字面值用b' '表示
            return i;
        }
    }
    s.len()

}

//返回字符串切片
fn first_worldqp(s:&String) -> &str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{        //空格的字面值用b' '表示
            return &s[..i];
        }
    }
    &s[..]
}


fn first_worldstr(s:&str) -> &str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{        //空格的字面值用b' '表示
            return &s[..i];
        }
    }
    &s[..]
}