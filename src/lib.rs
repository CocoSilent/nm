mod nodejs_manage;

use std::{error::Error, path::Path, fs, io::Write};

use nodejs_manage::nodejs_api;

pub struct Config {
    // exe_path: String,
    param1: String,
    param2: Option<String>,
    // param3: Option<String>,
}

pub const CONFIG_PATH: &str = "./config.json";


impl Config {
    pub fn build(mut args : impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 第一个参数是所在exe路径
        let _exe_path = match args.next() {
            Some(arg) => arg,
            None => return Err("exe_path获取失败"),
        };
        let param1 = match args.next() {
            Some(arg) => arg,
            None => return Err("命令获取失败"),
        };
        let param2 = args.next();
        // let param3 = args.next();
        Ok(Config {
            // exe_path,
            param1,
            param2,
            // param3
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut root = std::env::current_exe().unwrap();
    root.pop();
    std::env::set_current_dir(root);
    // 检查是否有config目录，没有则创建
    let path = Path::new(CONFIG_PATH);
    if !path.exists() {
        let mut f = fs::File::create(CONFIG_PATH)?;
        f.write_all("{\"used_version\":\"\",\"installed\":[]}".as_bytes())?;
    }
    if config.param1 == "install" {
        nodejs_api::install(config)?;
        Ok(())
    } else if config.param1 == "uninstall" || config.param1 == "remove" {
        nodejs_api::uninstall(config)?;
        Ok(())
    } else if config.param1 == "use" {
        nodejs_api::_use(config)?;
        Ok(())
    } else if config.param1 == "ls" || config.param1 == "list" {
        nodejs_api::ls()?;
        Ok(())
    } else if config.param1 == "current" {
        nodejs_api::current()?;
        Ok(())
    }  else if config.param1 == "v" || config.param1 == "version" {
        nodejs_api::version();
        Ok(())
    } else {
        println!("{}命令不支持", config.param1);
        Ok(())
    }
}