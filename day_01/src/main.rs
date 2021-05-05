fn main() {
    println!("Hello, world!");

    let a = 1;
    println!("原变量值：{}",a);
    let a = a+1;
    println!("声明同名变量隐藏a原来的值，去原来的值加1，最后值：{}",a);

    let mut m="hello";
    let mut n=m.clone();

    println!("m original value:{}",m);
    println!("n original value:{}",n);
    
    n="ok";
    println!("m original value:{}",m);
    println!("n original value:{}",n);

    
    
}
