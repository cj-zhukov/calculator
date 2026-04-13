mod calc;
mod repl;

fn main() {
    if let Err(e) = repl::run() {
        eprintln!("Fatal error: {}", e);
    }
}
