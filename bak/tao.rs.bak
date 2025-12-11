use std::{
    io::BufRead,
    sync::{Mutex, OnceLock},
};

// ------------------------------
// -------- global state --------
// ------------------------------
#[derive(Debug)]
struct TaoState {}
static TAO_STATE: OnceLock<Mutex<TaoState>> = OnceLock::new();

pub fn begin() -> Result<(), String> {
    let set_result = TAO_STATE.set(Mutex::new(TaoState {}));
    match set_result {
        Ok(_) => {}
        Err(_) => {
            let err_str = "[ERROR] global state was previously initialized, tao::begin() should not be called more than once!";
            return Err(err_str.to_string());
        }
    }

    Ok(())
}
pub fn end() -> Result<(), String> {
    let mut mutex_gaurd = TAO_STATE.get().unwrap().lock().unwrap();
    let _ = &mut *mutex_gaurd;

    Ok(())
}

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
fn _run_cmd(cmd: &mut std::process::Command) -> Result<(), String> {
    cmd.stdout(std::process::Stdio::piped());
    let mut child_process = cmd.spawn().map_err(|e| e.to_string())?;
    if let Some(output) = child_process.stdout.take() {
        let reader = std::io::BufReader::new(output);
        for line in reader.lines() {
            let line = match line {
                Ok(out) => out,
                Err(e) => {
                    return Err(e.to_string());
                }
            };
            debug::info(&line);
        }
    }
    Ok(())
}
pub fn install(target: &mut Target) -> Result<(), String> {
    match target {
        Target::Executable(exec) => {
            let mut cmd = std::process::Command::new(&exec.config.cc);
            cmd.arg("-std=c++23");
            cmd.arg(&exec.config.source_file);
            cmd.arg("-o")
                .arg(format!("{}/{}", &exec.config.build_dir, &exec.config.name));

            for dep in &exec.dependencies {
                let build_dir = format!("{}/{}", &dep.config.build_dir, &dep.config.name);
                let fmt_build_path = std::path::PathBuf::from(format!("{}/install", build_dir));
                match std::fs::create_dir_all(&fmt_build_path) {
                    Ok(_) => {}
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::AlreadyExists {
                            return Err(e.to_string());
                        }
                    }
                };

                cmd.arg(&format!("-I{}/install/include", &build_dir));
                cmd.arg(&format!("-L{}/install/lib", &build_dir));
                cmd.arg(&format!("-l{}", &dep.config.name));

                let mut target = Target::Library(dep.clone());
                install(&mut target)?;
            }
            _run_cmd(&mut cmd)?;
        }
        Target::Library(lib) => match lib.config.build_system {
            BuildSystem::CMAKE => {
                let build_dir = format!("{}/{}", &lib.config.build_dir, &lib.config.name);

                let mut cmd = std::process::Command::new("cmake");
                cmd.arg("-S").arg(&lib.config.source_dir);
                cmd.arg("-B").arg(&build_dir);
                cmd.arg(format!("-DCMAKE_INSTALL_PREFIX={}/install", &build_dir));
                cmd.arg("-DCMAKE_INSTALL_LIBDIR=lib");
                for i in &lib.config.extra_arguments {
                    cmd.arg(format!("-D{}={}", i.0, i.1));
                }
                _run_cmd(&mut cmd)?;

                let mut fmt_build_step = std::process::Command::new("cmake");
                fmt_build_step.arg("--build").arg(&build_dir);
                _run_cmd(&mut fmt_build_step)?;

                let mut fmt_install_step = std::process::Command::new("cmake");
                fmt_install_step.arg("--install").arg(&build_dir);
                _run_cmd(&mut fmt_install_step)?;
            }
        },
    }

    Ok(())
}
