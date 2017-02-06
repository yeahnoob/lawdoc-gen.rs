extern crate mustache;
extern crate rustc_serialize;
extern crate serde_yaml;

use std::io;
use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use mustache::MapBuilder;

fn read_string_from_file(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // get the template
    let tp = mustache::compile_str(&(read_string_from_file("./examples/t1.mustache").unwrap()))
        .unwrap();

    // read from YAML, fill every elements
    let de_map: HashMap<String, String> =
        serde_yaml::from_str(&(read_string_from_file("./examples/persons.yaml").unwrap())).unwrap();

    // print out result from the above contents & template
    println!(" * 合同生成 *");
    let mut mapdata = MapBuilder::new();
    for (id, text_content) in &de_map {
        mapdata = mapdata.insert_str(id, text_content);
    }
    tp.render_data(&mut io::stdout(), &(mapdata.build())).unwrap();
    println!("");
}
