use std::{thread, time};


pub fn hello_world() {  
    let hello_world = "Hello World";
    let fifteen_millis = time::Duration::from_millis(15);
    println!("{:?}",hello_world.as_bytes());
    let mut moving: [u8; 11] = [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32];
    let buffer: [u8; 11] = [72, 69, 76, 76, 79, 32, 87, 79, 82, 76, 68];
    let s = std::str::from_utf8(&buffer[..]).expect("invalid utf-8 sequence");
    println!("{}", s);
    for n in 0..11{
    while buffer[n] != moving[n] {
    moving[n] += 1;                                 
    let s2 = std::str::from_utf8(&moving[..]).expect("invalid utf-8 sequence");
    println!("{}", s2);
    thread::sleep(fifteen_millis);
    }
}
}