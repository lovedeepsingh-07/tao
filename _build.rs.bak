#[path = ".build/tao.rs"]
mod tao;

const BINARY_NAME: &str = "basement";
const EXTERNAL_DIR: &str = "deps";
const BUILD_DIR: &str = "build";
const TOOLCHAIN_FILE: &str = "mingw-w64-toolchain.cmake";
const CC: &str = "x86_64-w64-mingw32-g++";

fn configure_fmt() {
    let fmt_build_path = std::path::PathBuf::from(format!("{}/fmt/install", BUILD_DIR));
    match std::fs::create_dir_all(&fmt_build_path){
        Ok(_) => {},
        Err(e) => {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                println!("{}", e.to_string());
            }
        }
    };

    println!("configuring fmt...");
    let mut fmt_config_step = std::process::Command::new("cmake");
    fmt_config_step.arg("-S").arg(format!("{}/fmt", EXTERNAL_DIR));
    fmt_config_step.arg("-B").arg(format!("{}/fmt", BUILD_DIR));
    let toolchain_file_path = std::fs::canonicalize(std::path::PathBuf::from(TOOLCHAIN_FILE)).unwrap().to_string_lossy().to_string();
    fmt_config_step.arg(format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_file_path));
    fmt_config_step.arg("-DFMT_TEST=OFF");
    fmt_config_step.arg(format!("-DCMAKE_INSTALL_PREFIX={}/fmt/install", BUILD_DIR));
    fmt_config_step.output().unwrap();
}
fn build_fmt() {
    println!("building fmt...");
    let mut fmt_build_step = std::process::Command::new("cmake");
    fmt_build_step.arg("--build").arg(format!("{}/fmt", BUILD_DIR));
    fmt_build_step.output().unwrap();
}
fn install_fmt() {
    println!("installing fmt...");
    let mut fmt_install_step = std::process::Command::new("cmake");
    fmt_install_step.arg("--install").arg(format!("{}/fmt", BUILD_DIR));
    fmt_install_step.output().unwrap();
}

fn configure_raylib(){
    let raylib_build_path = std::path::PathBuf::from(format!("{}/raylib/install", BUILD_DIR));
    match std::fs::create_dir_all(&raylib_build_path){
        Ok(_) => {},
        Err(e) => {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                println!("{}", e.to_string());
            }
        }
    };

    println!("configuring raylib...");
    let mut raylib_config_step = std::process::Command::new("cmake");
    raylib_config_step.arg("-S").arg(format!("{}/raylib", EXTERNAL_DIR));
    raylib_config_step.arg("-B").arg(format!("{}/raylib", BUILD_DIR));
    let toolchain_file_path = std::fs::canonicalize(std::path::PathBuf::from(TOOLCHAIN_FILE)).unwrap().to_string_lossy().to_string();
    raylib_config_step.arg(format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_file_path));
    raylib_config_step.arg("-DBUILD_EXAMPLES=OFF");
    raylib_config_step.arg(format!("-DCMAKE_INSTALL_PREFIX={}/raylib/install", BUILD_DIR));
    raylib_config_step.output().unwrap();
}
fn build_raylib() {
    println!("building raylib...");
    let mut raylib_build_step = std::process::Command::new("cmake");
    raylib_build_step.arg("--build").arg(format!("{}/raylib", BUILD_DIR));
    raylib_build_step.output().unwrap();
}
fn install_raylib() {
    println!("installing raylib...");
    let mut raylib_install_step = std::process::Command::new("cmake");
    raylib_install_step.arg("--install").arg(format!("{}/raylib", BUILD_DIR));
    raylib_install_step.output().unwrap();
}
fn build() {
    println!("building {}...", BINARY_NAME);
    let mut build_step = std::process::Command::new(CC);
    build_step.arg("src/main.cpp");
    build_step.arg(format!("-I{}/fmt/install/include", BUILD_DIR));
    build_step.arg(format!("-L{}/fmt/install/lib", BUILD_DIR));
    build_step.arg("-lfmt");
    build_step.arg(format!("-I{}/raylib/install/include", BUILD_DIR));
    build_step.arg(format!("-L{}/raylib/install/lib", BUILD_DIR));
    build_step.arg("-lraylib");
    build_step.arg("-static").arg("-static-libstdc++").arg("-lgdi32").arg("-lwinmm");
    build_step.arg("-o").arg(format!("{}/{}", BUILD_DIR, BINARY_NAME));
    println!("{:#?}", build_step.output().unwrap());
}

fn main() {
    tao::setup();

    tao::create_cmd("hello", Box::new(|| {
        println!("tao is saying hello");
    }));
    tao::create_cmd("build", Box::new(|| {
        configure_fmt();
        build_fmt();
        install_fmt();
        configure_raylib();
        build_raylib();
        install_raylib();
        build();
    }));

    tao::run();
}
