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
#[derive(Debug, Clone)]
pub struct ExecutableConfig {
    pub cc: String,
    pub name: String,
    pub source_file: String,
    pub build_dir: String,
}
#[derive(Debug, Clone)]
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

pub fn create_executable(config: ExecutableConfig) -> Result<Executable, String> {
    Ok(Executable {
        config,
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

            let source_file_abs_path = std::fs::canonicalize(&exec.config.source_file).map_err(|e| e.to_string())?;
            ninja_file_str.push_str(&format!(
                "build {}: cc {}\n",
                exec.config.name, source_file_abs_path.to_string_lossy()
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
