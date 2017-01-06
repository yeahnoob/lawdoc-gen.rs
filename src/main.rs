extern crate mustache;
extern crate rustc_serialize;

use std::io;

#[derive(RustcEncodable)]
struct Planet {
    name: String,
}

fn main() {
    let temp = mustache::compile_str("您好 {{name}}").unwrap();
    let planet = Planet { name: "刘於凤".into() };
    println!(" * 生成结果 *");
    temp.render(&mut io::stdout(), &planet).unwrap();
    println!();
}
