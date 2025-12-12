extern crate tao;

fn build() -> Result<(), String> {
    let basement = tao::create_executable(tao::ExecutableConfig {
        cc: "gcc",
        binary: "basic_c",
        source_files: vec!["src/main.c", "src/app_utils.c", "src/context.c"],
        includes: vec!["include"],
        build_dir: "../../build/examples/basic_c",
        build_flags: vec!["-std=c23"],
        link_flags: vec![],
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
