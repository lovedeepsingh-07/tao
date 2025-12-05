use std::{
    collections::HashMap,
    io::BufRead,
    sync::{Mutex, OnceLock},
};

// ------------------------------
// -------- global state --------
// ------------------------------
#[derive(Debug)]
struct TaoState {
    cmd_list: HashMap<String, CmdCallback>,
    cmd_args: Vec<String>,
}
static TAO_STATE: OnceLock<Mutex<TaoState>> = OnceLock::new();

pub fn begin() -> Result<(), String> {
    let cmd_args = std::env::args()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>();

    let set_result = TAO_STATE.set(Mutex::new(TaoState {
        cmd_list: HashMap::new(),
        cmd_args,
    }));
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
    let TaoState { cmd_list, cmd_args } = &mut *mutex_gaurd;

    debug::debug(&format!("{:#?}", cmd_list));
    debug::debug(&format!("{:#?}", cmd_args));

    if let Some(cmd_arg) = cmd_args.get(1) {
        if let Some(run_cmd) = cmd_list.get(cmd_arg) {
            (run_cmd.0)();
        }
    }

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

// ----------------------------
// -------- commands ----------
// ----------------------------
#[allow(non_camel_case_types)]
pub type CmdCallback_Signature = Box<dyn Fn() + Send + Sync>;
pub struct CmdCallback(pub CmdCallback_Signature);

impl std::fmt::Debug for CmdCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback: Box<dyn Fn() + Send + Sync>")
    }
}

pub fn create_cmd(name: &str, callback: CmdCallback_Signature) {
    let mut mutex_gaurd = TAO_STATE.get().unwrap().lock().unwrap();
    let TaoState { cmd_list, .. } = &mut *mutex_gaurd;

    cmd_list.insert(String::from(name), CmdCallback(callback));
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
pub struct ExecutableConfig {
    pub cc: String,
    pub name: String,
    pub source_file: String,
    pub build_dir: String,
}
#[derive(Debug)]
pub struct Executable {
    cmd: std::process::Command,
    config: ExecutableConfig,
}
impl Executable {
    pub fn link_library(&mut self, lib: Library) -> Result<(), String> {
        let build_dir = format!("{}/{}", &lib.config.build_dir, &lib.config.name);
        self.cmd.arg(&format!("-I{}/install/include", &build_dir));
        self.cmd.arg(&format!("-L{}/install/lib", &build_dir));
        self.cmd.arg("-lfmt");
        install(&mut Target::Library(lib))?;
        Ok(())
    }
}

pub fn create_executable(config: ExecutableConfig) -> Result<Executable, String> {
    let mut cmd = std::process::Command::new(&config.cc);
    cmd.arg("-std=c++23");
    cmd.arg(&config.source_file);
    cmd.arg("-o")
        .arg(format!("{}/{}", &config.build_dir, &config.name));
    Ok(Executable { cmd, config })
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
            let _ = exec.config;
            debug::info(&format!("{:#?}", exec));
            _run_cmd(&mut exec.cmd)?;
        }
        Target::Library(lib) => {
            match lib.config.build_system {
                BuildSystem::CMAKE => {
                    let build_dir = format!("{}/{}", &lib.config.build_dir, &lib.config.name);

                    let mut cmd = std::process::Command::new("cmake");
                    cmd.arg("-S").arg(&lib.config.source_dir);
                    cmd.arg("-B").arg(&build_dir);
                    cmd.arg(format!("-DCMAKE_INSTALL_PREFIX={}/install", &build_dir));
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
            }
        }
    }

    Ok(())
}
