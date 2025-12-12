extern crate tao;

fn build() -> Result<(), String> {
    let basement = tao::create_executable(tao::ExecutableConfig {
        cc: String::from("g++"),
        binary: String::from("basic_cpp"),
        source_files: tao::utils::get_files_with_ext("src", "cpp")?,
        includes: tao::string_vec!("include"),
        build_dir: String::from("../../build/examples/basic_cpp"),
        build_flags: tao::string_vec!("-std=c23"),
        link_flags: tao::string_vec!(),
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
