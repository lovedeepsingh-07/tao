extern crate tao;

fn build() -> Result<(), String> {
    let basement = tao::create_executable(tao::ExecutableConfig {
        cc: "em++",
        binary: "basement.js",
        source_files: vec!["src/main.cpp"],
        includes: vec![],
        build_dir: "../../build/examples/basic_wasm",
        build_flags: vec!["-std=c++23", "-O3", "-Wall"],
        link_flags: link_flags,
    })?;
    tao::install(&mut tao::Target::Executable(basement))?;

    Ok(())
}

// fn build() -> Result<(), String> {
//     let mut link_flags = vec!["-lembind"];
//     link_flags.extend(vec![
//         "--no-entry",
//         "-sSTANDALONE_WASM",
//         "-sWASM_BIGINT",
//         "-sMODULARIZE",
//         "-sEXPORT_ES6",
//         "-sEXPORTED_RUNTIME_METHODS=['cwrap']",
//     ]);
//
//     let basement = tao::create_executable(tao::ExecutableConfig {
//         cc: "em++",
//         binary: "basement.js",
//         source_files: vec!["src/main.cpp"],
//         includes: vec![],
//         build_dir: "../../build/examples/basic_wasm",
//         build_flags: vec!["-std=c++23", "-O3", "-Wall"],
//         link_flags: link_flags,
//     })?;
//     tao::install(&mut tao::Target::Executable(basement))?;
//
//     Ok(())
// }

fn main() {
    match build() {
        Ok(_) => {}
        Err(e) => {
            tao::debug::error(&e.to_string());
        }
    }
}
