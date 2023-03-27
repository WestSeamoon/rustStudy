fn main() {
    another_function(5,6);        //argument
    let a=five();
    let b=plus_five(6);
    println!("five()和plus_five()的返回值分别为:{},{}",a,b);

    //语句和表达式
    //let x = (let y=6);        会报错，let期待的的是表达式
    let x = 5;
    let y = {
        let x=1;
        x+2        //如果加了分号，则是一个语句，语句没有返回值或返回值是一个（)所以无法打印
    };
    println!("{},{}",x,y);

    //在let语句中使用if（if是表达式）
    let condition = true;
    let c = 1>2;
    let number = if condition {5} else {6};
    let number1 = if c {5} else {6};
    println!("在let语句中使用if:{},{}",number,number1);

    //循环
    let mut counter = 0;
    
    //loop
    let result = loop {
        counter += 1;
        
        if counter == 3 {
            break counter*2;
        }
    };
    
    println!("loop循环的结果为：{}",result);

    //while
    while counter != 0 {
        println!("while循环过程{}!",counter);
        counter = counter - 1;
    }
    println!("LIFTOFF!!!");

    //for循环
    let d=[10,20,30,40,50];
    for element in d.iter(){            //iter方法，返回一个可以遍历的东西
        println!("for循环内容:{}",element);
    }
    
    //Range 标准库提供
    //指定一个开始和一个结束数字，Range可以生成它们之间的数字（不含结束）
    //rev方法可以反转Range
    for number in (1..4).rev(){
        println!("Range,rev方法:{}!",number);
    }
    println!("LIFTOFF!!!");

}


//声明在main函数前后都可以
fn another_function(x:i32,y:i32){     //parameter
    println!("the valueof x is:{}",x);
    println!("the valueof x is:{}",y);
}

fn five() -> i32{
    5                       //默认使用最后一个表达式作为返回值
}

fn plus_five(x:i32) -> i32 {
    x+5
}