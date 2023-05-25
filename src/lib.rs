mod nodejs_manage;

use std::error::Error;

use nodejs_manage::nodejs_api;

pub struct Config {
    exe_path: String,
    param1: String,
    param2: Option<String>,
    param3: Option<String>,
}

impl Config {
    pub fn build(mut args : impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 第一个参数是所在exe路径
        let exe_path = match args.next() {
            Some(arg) => arg,
            None => return Err("exe_path获取失败"),
        };
        let param1 = match args.next() {
            Some(arg) => arg,
            None => return Err("param1获取失败"),
        };
        let param2 = args.next();
        let param3 = args.next();
        Ok(Config {
            exe_path,
            param1,
            param2,
            param3
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.param1 == "install" {
        nodejs_api::install(config)?;
        Ok(())
    } else if config.param1 == "use" {
        nodejs_api::_use(config);
        Ok(())
    } else {
        println!("{}命令不支持", config.param1);
        Ok(())
    }
}