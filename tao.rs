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
    fn does_command_exist(cmd: &str) -> Option<String> {
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
    fn does_file_exist(file: &str) -> Option<String> {
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
#[derive(Debug, Clone)]
pub enum BuildSystem {
    CMAKE,
}

// -------------------------
// -------- library --------
// -------------------------
#[derive(Debug, Clone)]
pub struct LibraryConfig {
    pub build_system: BuildSystem,
    pub name: String,
    pub source_dir: String,
    pub build_dir: String,
    pub extra_arguments: Vec<(String, String)>,
}
#[derive(Debug, Clone)]
pub struct Library {
    config: LibraryConfig,
}
pub fn create_library(config: LibraryConfig) -> Result<Library, String> {
    Ok(Library { config })
}

// ----------------------------
// -------- executable --------
// ----------------------------
#[derive(Debug, Clone, Default)]
pub struct ExecutableConfig {
    pub cc: String,
    pub name: String,
    pub source_file: String,
    pub build_dir: String,
}
#[derive(Debug, Clone, Default)]
pub struct Executable {
    config: ExecutableConfig,
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
    let mut final_config = ExecutableConfig::default();

    // check if `cc` command exists
    match does_command_exist(&config.cc) {
        Some(cmd) => {
            final_config.cc = cmd;
        }
        None => {
            return Err(format!("executable {:#?} does not exist", &config.cc));
        }
    }

    // check if `source_file` exists
    // TODO: this does not check if `source_file` is a file
    match does_file_exist(&config.source_file) {
        Some(file_path) => {
            final_config.source_file = file_path;
        }
        None => {
            return Err(format!("file {:#?} does not exist", &config.source_file));
        }
    }

    // check if `build_dir` exists
    // TODO: if `build_dir` does not exist, then we must create it
    match does_file_exist(&config.build_dir) {
        Some(file_path) => {
            final_config.build_dir = file_path;
        }
        None => {
            return Err(format!("folder {:#?} does not exist", &config.build_dir));
        }
    }

    final_config.name = config.name;
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
            ninja_file_str.push_str("rule cc\n");
            ninja_file_str.push_str(&format!("    command = {} $in -o $out\n", exec.config.cc));

            let source_file_abs_path =
                std::fs::canonicalize(&exec.config.source_file).map_err(|e| e.to_string())?;
            ninja_file_str.push_str(&format!(
                "build {}: cc {}\n",
                exec.config.name,
                source_file_abs_path.to_string_lossy()
            ));
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
