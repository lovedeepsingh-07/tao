#[path = ".build/tao.rs"]
mod tao;

fn build() -> Result<(), String> {
    tao::begin()?;

    let mut basement = tao::create_executable(tao::ExecutableConfig {
        cc: "g++".to_string(),
        name: "basement".to_string(),
        source_file: "src/main.cpp".to_string(),
        build_dir: "build".to_string(),
    })?;

    let mut fmt_lib = tao::create_library(tao::LibraryConfig {
        build_system: tao::BuildSystem::CMAKE,
        name: "fmt".to_string(),
        source_dir: "deps/fmt".to_string(),
        build_dir: "build".to_string(),
        extra_arguments: vec![("FMT_TEST".to_string(), "OFF".to_string())],
    })?;

    basement.link_library(fmt_lib)?;
    tao::install(&mut tao::Target::Executable(basement))?;

    tao::end()?;
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
