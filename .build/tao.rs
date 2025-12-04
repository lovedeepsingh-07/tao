use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

// ------ global state ------
#[derive(Debug)]
struct TaoState {
    cmd_list: HashMap<String, CmdCallback>,
    cmd_args: Vec<String>,
}
static TAO_STATE: OnceLock<Mutex<TaoState>> = OnceLock::new();

pub fn begin() -> Result<(), String> {
    let cmd_args = std::env::args().map(|arg| {
        arg.to_string()
    }).collect::<Vec<String>>();

    let set_result = TAO_STATE.set(Mutex::new(TaoState {
        cmd_list: HashMap::new(),
        cmd_args,
    }));
    match set_result {
        Ok(_) => {},
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

    println!("[DEBUG] {:#?}", cmd_list);
    println!("[DEBUG] {:#?}", cmd_args);

    if let Some(cmd_arg) = cmd_args.get(1){
        if let Some(run_cmd) = cmd_list.get(cmd_arg) {
            (run_cmd.0)();
        }
    }

    Ok(())
}

// ------ commands ------
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


// ------ executables ------
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
pub fn create_executable(config: ExecutableConfig) -> Result<Executable, String> {
    let mut cmd = std::process::Command::new(&config.cc);
    cmd.arg(&config.source_file);
    cmd.arg("-o")
        .arg(format!("{}/{}", &config.build_dir, &config.name));
    Ok(Executable { cmd, config })
}
pub fn install(exec: &mut Executable) -> Result<(), String> {
    let _ = exec.cmd.output().map_err(|e| e.to_string());
    let _ = exec.config;
    Ok(())
}
