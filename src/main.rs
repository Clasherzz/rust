use std::io;

fn add() -> i64 {
    let n = 2;
    let mut n1 = 3;
    n1 = n + n1;
    n1
}
fn input(){
    println!("enter input");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("error reading line");
    println!("{}",s);
}
fn factorial(x: i64)->i64{
    if x==1{
        1
    }else{
        x*factorial(x-1)

    }
}
pub mod my_module{
    pub mod sub_mod{
    pub fn hai(){
        println!("hai");
    }
    }
}

use my_module::sub_mod::hai;
 
fn main() {

    println!("Hello, world!");
    let m = factorial(5);
    println!("{}",m);
    //println!("{}",even(2));
    hai();
    for i in 1..10{
        println!("{}",i);
    }
    
}
