fn main() {
    let _x:u32=44;
    let _f:f32=44.5;
    let _bool:bool=true;
    let _char:char='z';
    let _tuple:(i32,f32,u8)=(500,4.5,8);
    let tup2=(100,10,5);
    let (a1,a2,a3)=tup2;
    // _x=33;
    let x=_tuple.0;


    println!("The value of x is {_x}");
    println!("The value of f is {_f}");
    println!("The value of bool is {_bool}");
    println!("{}",x);   
    println!("{}",a1);   
    println!("{}",a2);   
    println!("{}",a3);   
    


    
}
