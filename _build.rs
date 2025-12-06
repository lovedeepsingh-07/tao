#[path = ".build/tao.rs"]
mod tao;

fn build() -> Result<(), String> {
    Ok(())
}

fn main() {
    match build() {
        Ok(_) => {}
        Err(e) => {
            tao::debug::error(&e.to_string());
        }
    }
}
