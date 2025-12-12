// -----------------------
// -------- debug --------
// -----------------------
#[allow(dead_code)]
pub mod debug {
    pub fn debug(input: &str) {
        let flag_str = "\x1b[90m[DEBUG]\x1b[0m";
        println!("{} {}", flag_str, input);
    }
    pub fn info(input: &str) {
        let flag_str = "\x1b[94m[INFO]\x1b[0m";
        println!("{} {}", flag_str, input);
    }
    pub fn warn(input: &str) {
        let flag_str = "\x1b[93m[WARN]\x1b[0m";
        println!("{} {}", flag_str, input);
    }
    pub fn error(input: &str) {
        let flag_str = "\x1b[91m[ERROR]\x1b[0m";
        println!("{} {}", flag_str, input);
    }
}

// -----------------------
// -------- utils --------
// -----------------------
pub mod utils {
    // ------ does_command_exist ------
    pub fn does_command_exist(cmd: &str) -> Option<String> {
        match std::process::Command::new("which").arg(cmd).output() {
            Ok(output) => {
                let cmd_path = output.stdout;
                if cmd_path.is_empty() {
                    return None;
                }
                return Some(String::from_utf8(cmd_path).unwrap().trim().to_string());
            }
            _ => {
                return None;
            }
        };
    }

    // ------ does_file_exist ------
    pub fn does_file_exist(file: &str) -> Option<String> {
        let file_path = std::path::Path::new(file);
        if file_path.exists() {
            return Some(
                std::fs::canonicalize(file)
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            );
        }
        return None;
    }
}

// ------------------------
// -------- target --------
// ------------------------
#[derive(Debug)]
pub enum Target {
    Executable(Executable),
    Library(Library),
}

// ------------------------------
// -------- build system --------
// ------------------------------
#[derive(Debug)]
pub enum BuildSystem {
    CMAKE,
}

// -------------------------
// -------- library --------
// -------------------------
#[derive(Debug)]
pub struct LibraryConfig {
    pub build_system: BuildSystem,
    pub name: String,
    pub source_dir: String,
    pub build_dir: String,
    pub extra_arguments: Vec<(String, String)>,
}
#[derive(Debug)]
pub struct Library {
    config: LibraryConfig,
}
pub fn create_library(config: LibraryConfig) -> Result<Library, String> {
    Ok(Library { config })
}

// ----------------------------
// -------- executable --------
// ----------------------------
#[derive(Debug)]
pub struct ExecutableConfig<'a> {
    pub cc: &'a str,
    pub name: &'a str,
    pub source_files: Vec<&'a str>,
    pub includes: Vec<&'a str>,
    pub build_dir: &'a str,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
struct Internal_ExecutableConfig {
    pub cc: String,
    pub name: String,
    pub source_files: Vec<String>,
    pub includes: Vec<String>,
    pub build_dir: String,
}
#[derive(Debug, Default)]
pub struct Executable {
    config: Internal_ExecutableConfig,
    dependencies: Vec<Library>,
}
impl Executable {
    pub fn link_library(&mut self, lib: Library) -> Result<(), String> {
        self.dependencies.push(lib);
        Ok(())
    }
}

// ------ create_executable ------
pub fn create_executable(config: ExecutableConfig) -> Result<Executable, String> {
    let mut final_config = Internal_ExecutableConfig::default();

    // check if `cc` command exists
    match utils::does_command_exist(config.cc) {
        Some(cmd) => {
            final_config.cc = cmd;
        }
        None => {
            return Err(format!("executable {:#?} does not exist", config.cc));
        }
    }

    // check if `source_files` exists
    // TODO: this does not check if it is a file
    for curr_file in config.source_files {
        match utils::does_file_exist(curr_file) {
            Some(file_path) => {
                final_config.source_files.push(file_path.to_string());
            }
            None => {
                return Err(format!("file {:#?} does not exist", curr_file));
            }
        }
    }

    // check if `includes` exists
    // TODO: this does not check if it is a folder
    for curr_folder in config.includes {
        match utils::does_file_exist(curr_folder) {
            Some(file_path) => {
                final_config.includes.push(file_path.to_string());
            }
            None => {
                return Err(format!("folder {:#?} does not exist", curr_folder));
            }
        }
    }

    // check if `build_dir` exists
    // TODO: if `build_dir` does not exist, then we must create it
    match utils::does_file_exist(config.build_dir) {
        Some(file_path) => {
            final_config.build_dir = file_path;
        }
        None => {
            return Err(format!("folder {:#?} does not exist", config.build_dir));
        }
    }

    final_config.name = String::from(config.name);
    Ok(Executable {
        config: final_config,
        dependencies: Vec::new(),
    })
}

// ------------------------------
// -------- installation --------
// ------------------------------
pub fn install(target: &mut Target) -> Result<(), String> {
    match target {
        Target::Executable(exec) => {
            let mut ninja_file_str = String::new();

            ninja_file_str.push_str(&format!("BUILD_DIR = {}\n", &exec.config.build_dir));
            ninja_file_str.push_str(&format!("BINARY = {}\n", &exec.config.name));
            ninja_file_str.push_str("BUILD_FLAGS = -std=c23\n");
            ninja_file_str.push_str("INCLUDES = ");
            for curr_include in &exec.config.includes {
                ninja_file_str.push_str(&format!("-I{}", curr_include));
            }
            ninja_file_str.push_str("\n");
            ninja_file_str.push_str("\n");

            ninja_file_str.push_str("rule cc\n");
            ninja_file_str.push_str("    depfile = $out.d\n");
            ninja_file_str.push_str(&format!(
                "    command = {} -MD -MF $out.d ${{BUILD_FLAGS}} ${{INCLUDES}} -c $in -o $out\n",
                exec.config.cc
            ));

            ninja_file_str.push_str("rule link\n");
            ninja_file_str.push_str(&format!("    command = {} $in -o $out\n", exec.config.cc));
            ninja_file_str.push_str("\n");

            for curr_file in &exec.config.source_files {
                let source_file_path = std::path::PathBuf::from(&curr_file);
                let mut obj_file_path = source_file_path.clone();
                obj_file_path.set_extension("o");
                ninja_file_str.push_str(&format!(
                    "build ${{BUILD_DIR}}/obj/{}: cc {}\n",
                    obj_file_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                    &curr_file
                ));
            }
            ninja_file_str.push_str("\n");

            ninja_file_str.push_str("build ${BUILD_DIR}/${BINARY}: link ");
            for curr_file in &exec.config.source_files {
                let mut obj_file_path = std::path::PathBuf::from(&curr_file);
                obj_file_path.set_extension("o");
                ninja_file_str.push_str(&format!(
                    "${{BUILD_DIR}}/obj/{} ",
                    obj_file_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                ));
            }
            ninja_file_str.push_str("\n");

            std::fs::write(
                format!("{}/build.ninja", exec.config.build_dir),
                ninja_file_str,
            )
            .map_err(|e| e.to_string())?;
        }
        Target::Library(lib) => match lib.config.build_system {
            BuildSystem::CMAKE => {}
        },
    }

    Ok(())
}
