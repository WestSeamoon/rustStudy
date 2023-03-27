enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //创建Vector
    let v1: Vec<i32> = Vec::new();      //new没有东西所以要写清楚类型
    let v2 = vec![1,2,3];
    
    let mut v3 = Vec::new();            //后面没有插入值，会报错，因为无法识别类型
    v3.push(1);

    //所有权和借用规则：不能在同一作用域内同时拥有可变和不可变引用
    let first = &v3[0];                 //不可变引用
    //v3.push(2);                         //可变引用，会报错
    println!("The first elemt is {}",first);

    //读取Vector的元素
    let third: &i32 = &v2[2];           //索引方法读取,如果索引超出范围，程序会panic崩溃
    println!("The third element is {}",third);

    match v2.get(2) {                   //如果索引超出范围，程序不会panic，值就是None
        Some(third) => println!("The third element is {}",third),       //这个third与上面的third无关，写成其它字母也可以
        None => println!("There is no third elemrnt"),

    }

    //遍历Vector里中的值
    for i in &v2 {                      //使用不可变的引用来遍历
        println!("{}",i);
    }

    for i in &mut v3 {                  //可变引用
        *i += 50;                       //使用*解引用
    }
    for i in v3 {                       //遍历改变以后的Vector
        println!("{}",i);
    }

    //使用可附加数据的枚举来创建Vector
    let v4 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

}
