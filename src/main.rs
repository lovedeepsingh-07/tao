fn main() {
    let curr_working_dir = "example";
    let build_dir = "build";
    let binary_name = "main";
    let src_file = "src/main.c";

    let build_path = std::path::PathBuf::from(format!("{}/{}/{}", curr_working_dir, build_dir, binary_name));
    match std::fs::create_dir(&build_path){
        Ok(_) => {},
        Err(e) => {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                eprintln!("failed to create directory {}: {}", build_path.to_string_lossy(), e.to_string());
            }
        }
    };

    let mut build_step = std::process::Command::new("gcc");
    build_step.current_dir(curr_working_dir);
    build_step.arg(src_file);
    build_step.arg("-o").arg(format!("{}/{}", build_dir, binary_name));
    build_step.output().unwrap();

    let mut run_step = std::process::Command::new(format!("./{}/{}", build_dir, binary_name));
    run_step.current_dir(curr_working_dir);
    let run_step_output = run_step.output().unwrap();
    println!("{}", String::from_utf8(run_step_output.stdout).unwrap().trim());
}
