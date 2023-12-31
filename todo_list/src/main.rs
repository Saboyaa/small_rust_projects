use std::io;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]

struct Item{
    name: String,
    normal_price: u32,
    desired_ammount: u32,
}
fn read(x:String) -> String {
    let mut z = String::new();
    let print_format = format!("What is the item {atribute}",atribute = x);
    println!("{:?}",print_format);
    io::stdin().read_line(&mut z).expect("Failed");
    return z.trim().parse().expect("Error to parse");
}

fn register_product() -> Item{
    let name:String = read("name".to_string());
    let normal_price:u32 = read("normal_price".to_string()).parse().unwrap();
    let desired_ammount:u32 = read("desired_ammount".to_string()).parse().unwrap();
    let product = Item {
        name:name,
        normal_price:normal_price,
        desired_ammount:desired_ammount
    };
    return product;
}

fn main() -> std::io::Result<()> {
    let contents = read_to_string("products.json")
        .expect("Should have been able to read the file");
    println!("{}",contents);
    let mut deserialized: Vec<Item> = serde_json::from_str(&contents).unwrap();
    deserialized.push(register_product());
    let serialized = serde_json::to_string(&deserialized).unwrap();
    println!("{}",serialized);
    let mut data_file = File::create("products.json")?;
    writeln!(&mut data_file,"{}", serialized)?;
    Ok(())
}