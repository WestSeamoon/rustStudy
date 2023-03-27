//常量也可以在这定义，可使用下划线增加代码可读性
const MAX_POINTS:u32=10_000;

fn main() {
    //变量与常量：
    let a=0;
    let mut b=1;
    const MIN_POINTS:u32=10;

    //shadowing
    let x=5;
    let x=x+1;
    //使用let声明的同名新变量，类型可以与之前不同
    let spaces = "   ";
    let spaces = spaces.len();

    //浮点数
    let c=2.0;  // f64
    let d:f32=3.0;  //f32

    //数值操作,两个数必须同类型才能操作
    //支持对浮点数取余，两边都是同浮点数才行
    let reminder = 54.0 % 5.1;
    println!("{}",reminder);

    //bool类型
    let t = true;
    let f:bool = false;

    //字符类型
    let e = 'z';
    let f:char = '？';  //不管什么语言的字符
    let g = 'x';        //也可以是表情，依然是字符类型

    //复合类型
    let tup:(i32,f64,u8) = (500,6.4,1);     //定义一个tuple
    println!("{},{},{}",tup.0,tup.1,tup.2); //访问Tuple的元素
    let (h,i,j) = tup;                      //解构 ，获取Tuple的元素值   
    println!("{},{},{}",h,i,j);             

    //数组
    let k = [1,2,3,4,5];
    let months = [
        "Jsnuary",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    //数组的类型
    let l:[i32;5]=[1,2,3,4,5];
    let m=[3;5];//相当于定义了一个长度为5，值全是3的数组
    //访问数组的元素
    let first = months[0];
    //访问超出数组范围
    //let a = months[12];        VS直接报错，编译和执行都报错
    let index = [12,13,14,15];
    let month = months[index[1]];   //s和编译都不报错，执行报错
}
