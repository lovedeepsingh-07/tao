use std::io::BufRead;

fn main(){
    let mut cmd = std::process::Command::new("./test.sh");
    cmd.stdout(std::process::Stdio::piped());
    let mut child = cmd.spawn().unwrap();

    if let Some(output) = child.stdout.take(){
        let mut reader = std::io::BufReader::new(output);
        for line in reader.lines(){
            println!("{}", line.unwrap());
        }
    }
}
