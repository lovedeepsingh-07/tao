use std::fs;
use tao;

fn main() {
    let parsed_tao = tao::parse(fs::read_to_string("./demo/tao.tao").unwrap());
    println!("{:#?}", parsed_tao);
}
