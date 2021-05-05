## RUST工具
cargo 构建工具、依赖管理工具
rustfmt 代码风格一致
rust language server

## 宏（marco）
println!() 
调用宏使用`！`符号，否则则是调用普通函数

## cargo 
cargo 默认源码目录根目录下的src文件夹
`cargo new hello`
新建名为`hello`的格式化项目
`cargo init `
在当前目录初始化一个项目
`cargo build `
编译当前包
`cargo doc`
解析当前包报告错误，但~~构建目标文件~~
`cargo update`
更新Cargo.lock中的依赖
## 变量和可变性
let 声明变量默认是值不可变的
额外添加mut关键词声明可以改变变量的值
常量是使用 `const`关键字声明
        RUST常量规范，大写字母中用下划线分隔
[变量隐藏](https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html#%E9%9A%90%E8%97%8Fshadowing)


## 所有权&引用
### 引用权
所有权规则：
        每个值都有一个称为拥有者的变量
        每个值任一时刻只有一个拥有者
        拥有者离开作用域时，值被抛弃

所有权的几个操作：移动、复制

```
let a= "hello";
let b= a;//a的所有权失效

println!("{}",a);//报错，a所有权失效了
```
上述仅发生了所有权的移动，
当"hello"的所有权由a移交给b时，
rust则规定a此时失效，无法使用a

```
let a= "hello";
let b=a.clone();
```
clone()能够完成所有权的复制，复制了堆上的内存

如若是可变变量改变时两个相互之间不会影响

函数使用所有权进行参数传递时，
所有权移动后如若不在将所有权移出值将会在函数内部被抛弃

为传递参数而不需要移动所有权，引用很好的实现了
符号`&`，表示使用引用，引用默认不可变
`&mut`，可变引用
如果值已经被抛弃，则会出现悬空引用。


