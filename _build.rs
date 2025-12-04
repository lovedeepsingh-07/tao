#[path = ".build/tao.rs"]
mod tao;

fn build() -> Result<(), String> {
    tao::begin()?;

    tao::create_cmd("build", Box::new(|| {
        println!("tao is saying hello!");
    }));

    tao::create_cmd("build", Box::new(|| {
        println!("tao is saying hello but again!");
    }));

    let mut exec_c = tao::create_executable(tao::ExecutableConfig {
        cc: "gcc".to_string(),
        name: "basement_c".to_string(),
        source_file: "src/main.c".to_string(),
        build_dir: "build".to_string(),
    })?;
    tao::install(&mut exec_c)?;

    let mut exec_cpp = tao::create_executable(tao::ExecutableConfig {
        cc: "g++".to_string(),
        name: "basement_cpp".to_string(),
        source_file: "src/main.cpp".to_string(),
        build_dir: "build".to_string(),
    })?;
    tao::install(&mut exec_cpp)?;

    tao::end()?;
    Ok(())
}

fn main() {
    match build() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.to_string());
        }
    }
}
