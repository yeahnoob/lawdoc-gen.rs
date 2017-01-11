extern crate mustache;
extern crate rustc_serialize;

use std::io;
use mustache::{MapBuilder, Template, Data};

#[derive(RustcEncodable)]
struct Planet {
    name: String,
    sex: String,
}

fn data_stdout(tp: &Template, dp: &Data) {
    println!("===");
    tp.render_data(&mut io::stdout(), &dp).unwrap();
    println!();
}

fn planet_stdout(tp: &Template, dp: &Planet) {
    println!("===");
    tp.render(&mut io::stdout(), &dp).unwrap();
    println!();
}

fn main() {
    // read the template
    let tp = mustache::compile_str("用户: {{name}}\n性别: {{sex}}").unwrap();
    // fill contents
    let planet = Planet {
        name: "刘於凤".into(),
        sex: "女".into(),
    };
    // get result from the above contents & template
    println!(" * 生成结果 *");
    planet_stdout(&tp, &planet);

    // closures
    let mut names = vec!["甲", "乙", "丙"];
    let mut sexs = vec!["男", "男", "女"];
    let data = MapBuilder::new()
        .insert_fn("name", move |_| names.pop().unwrap().into())
        .insert_fn("sex", move |_| sexs.pop().unwrap().into())
        .build();

    data_stdout(&tp, &data);
    data_stdout(&tp, &data);
    data_stdout(&tp, &data);
}
