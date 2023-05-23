use nm::{Config, run};

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(| err | {
        eprintln!("{err}");
        std::process::exit(1);
    });
    run(config);
}
