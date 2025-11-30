use std::{sync::{Mutex, OnceLock}, collections::HashMap};

#[allow(non_camel_case_types)]
pub type Cmd_Callback_Alias = Box<dyn Fn() + Send + Sync>;

#[allow(non_camel_case_types)]
struct Cmd_Callback(pub Cmd_Callback_Alias);
impl std::fmt::Debug for Cmd_Callback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Box<dyn Fn()>")
    }
}

#[allow(non_camel_case_types)]
struct Tao_State {
    cmd_list: HashMap<String, Cmd_Callback>,
    cmd_args: Vec<String>,
}

static TAO_STATE: OnceLock<Mutex<Tao_State>> = OnceLock::new();

pub fn setup() {
    let cmd_args = std::env::args().map(|arg| {
        arg.to_string()
    }).collect::<Vec<String>>();

    TAO_STATE
        .set(Mutex::new(Tao_State {
            cmd_list: HashMap::new(),
            cmd_args,
        }))
        .ok();
}

pub fn create_cmd(name: &str, callback: Cmd_Callback_Alias) {
    let mut mutex_gaurd = TAO_STATE.get().unwrap().lock().unwrap();
    let Tao_State { cmd_list, .. } = &mut *mutex_gaurd;

    cmd_list.insert(String::from(name), Cmd_Callback(callback));
}

pub fn run() {
    let mut mutex_gaurd = TAO_STATE.get().unwrap().lock().unwrap();
    let Tao_State { cmd_list, cmd_args } = &mut *mutex_gaurd;

    if let Some(cmd_arg) = cmd_args.get(1){
        if let Some(run_cmd) = cmd_list.get(cmd_arg) {
            (run_cmd.0)();
        }
    }
}
