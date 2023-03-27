fn main() {
    
    //创建String类型
    let mut s = String::from("hello");

    s.push_str(",world");

    //针对heap上的数据
    //let s2=s;         浅拷贝s1会失效
    let s2 = s.clone();     //对String数据进行深度拷贝可以使用克隆
    println!("String类型输出{},{}",s,s2);

    //针对stack上的数据：拷贝（Copy trait）
    let x = 5;
    let y = x;

    println!("Stack上的拷贝{},{}",x,y);


    take_ownership(s);  //s所有权给出去，离开作用域，在main函数此后不再有效
    makes_copy(x);  //x后面依然有效
    println!("copy以后的x:{}",x);

    let s3 = gives_ownership();         //获得字符串所有权
    let s4 = String::from("Hello");
    let s5 = takes_and_gives_back(s4);      //字符串所有权给出去再拿回来

    let (s6,len) = calculate_length(s5);
    println!("The length of {} is {}",s6,len);

    //引用和借用
    let len1 = calculate_length1(&s6);
    println!("The length of {} is {}",s6,len1);

    //可变引用
    let mut s7 = String::from("Hello");
    let len7 = calculate_length7(&mut s7);
    println!("The length of {} is {}",s7,len7);

    //在特定作用域内，对于某一块数据，只能有一个可变引用
    let mut s8 = String::from("Hello");
    let s9 = &mut s8;
    //let s10 = &mut s8;            这里会报错
    
    //可以通过创建新的作用域，来允许非同时的创建多个引用
    {
        let s11 = &mut s8;
    }

    //不可以同时拥有一个可变引用和一个不变引用
    let mut s12 = String::from("Hello");
    let s13 = &s12;
    let s14 = &s12;
    //let mut s15 = &mut s12;       这样就会报错

    println!("同时拥有可变引用和不可变引用:{},{}",s12,s13);

    //悬空引用Dangling References
    let r = dangle();

}

fn take_ownership(some_string:String){
    println!("take_owner里的String:{}",some_string);
}

fn makes_copy(some_number:i32){
    println!("makes_copy里的number:{}",some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}

fn calculate_length(b_string:String) -> (String,usize){     //usize索引类型的无符号整数类型
    let length = b_string.len();
    (b_string,length)                       //返回一个副本

}

fn calculate_length1(s:&String) -> usize {
    //s.push_str("world");                  借用：把引用作为函数参数，不能修改借用的东西
    s.len()
}

fn calculate_length7(s: &mut String) -> usize {
    s.push_str(",world!");
    s.len()
}

fn dangle() -> &String{                     //可以返回，但是要生命周期
    let s = String::from("Hello");
    &s
}