extern crate tao;

fn build() -> Result<(), String> {
    let basement = tao::create_executable(tao::ExecutableConfig {
        cc: "gcc".to_string(),
        name: "basement".to_string(),
        source_file: "src/main.c".to_string(),
        build_dir: "../../build/examples/basic_c".to_string(),
    })?;
    tao::install(&mut tao::Target::Executable(basement))?;
    Ok(())
}

fn main() {
    match build() {
        Ok(_) => {}
        Err(e) => {
            tao::debug::error(&e.to_string());
        }
    }
}
