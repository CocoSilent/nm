use nm::{Config, run};

fn main() {
    let config = Config::build(std::env::args()).unwrap_or_else(| err | {
        eprintln!("{err}");
        std::process::exit(1);
    });
    if let Err(err) = run(config) {
        eprintln!("执行命令发生错误：{err}");
        std::process::exit(1);
    }
}
