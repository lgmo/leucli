use std::process;

fn main(){
    if let Err(e) = leucli::run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}
