//#[derive(Debug)];
//use std::io;
use rand::Rng;                  //trait
use std::cmp::Ordering;

pub struct Guess {
    value:i32,
}


impl Guess {
    pub fn new (value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic !("Guess value must be between 1 and 100,got {}", value);
        }
        Guess {value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("猜数游戏!");
    let secret_number:i32=rand::thread_rng().gen_range(1,101);      //i32 u32 i64

    //println!("神秘数字是{}",secret_number);

    loop{
        println!("猜测一个数");

        //let foo = 1;                              //不加mut则变量不可修改，immutable
        //foo = 2;      会报错

        let mut guess = String::new();              //String::new()空字符串实例

        std::io::stdin().read_line(&mut guess).expect("无法读取行");    //不加会有警告但能运行
        //io:Result Ok,Err

        //shadow,允许使用同一变量名不会报错，用于类型转换，后面指转换后的变量
        //trim把前后的空白去掉（包括回车），parse解析成u32类型,parse返回一个枚举类型result，枚举类型可以使用match
        //let guess:u32=guess.trim().parse().expect("Please type a number!");
        let guess:i32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        /*
        //如果猜测的数字超出范围
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100");
            continue;
        }
        */


        //为验证创建自定义类型，效果等同于以上注释
        let guess = Guess::new(guess);

        

        println!("你猜测的数是：{}",guess.value);                  //如果用Guess则是打印结构体
    
        match guess.value().cmp(&secret_number){                    //结构体与数字比较会报错
            Ordering::Less =>println!("Too small!"),        //arm
            Ordering::Greater =>println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            },
        }

    }

    
}
