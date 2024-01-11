fn main() {
    let_method();
    let_mut_method();
    const_method();
    shadowing_method();
}

fn let_method() {
    let name= "苗子说全栈";
    println!("Hello, {name}");

    //error[E0384]: cannot assign twice to immutable variable `name`
    //name = "覆盖上面的名字";
    //println!("Hello, {name}");
}

fn let_mut_method() {
    let mut name= "苗子说全栈";
    println!("Hello, {name}");
    name = "覆盖上面的名字";
    println!("Hello, {name}");
}

fn const_method(){
    const FIRST_CONST : u32 = 100 * 10;
    println!("常量的值： {FIRST_CONST}");
}

fn shadowing_method(){
    let x = 5 ;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x: 当前的值 {x}");
    }
    println!("x 外部的值：{x}")
}
