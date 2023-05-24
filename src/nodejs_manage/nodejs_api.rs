use std::error::Error;
use serde::{Deserialize};

use crate::Config;

#[derive(Debug, Deserialize)]
// https://nodejs.org/dist/index.json返回的结构
struct Version_info {
    version: String,
}

// 版本号
struct Version_number {
    one: String,
    two: String,
    three: String,
    version: String,
}

impl Version_number {
    fn parse(version: String) -> Result<Version_number, String> {
        let arr: Vec<_> = version.split('.').collect();
        if arr.len() > 3 {
            let str = format!("版本号：{version},格式不正确");
            return Err(str);
        }
        Ok(Version_number { one: String::new(), two: String::new(), three: String::new(), version })
    }
}

// https://nodejs.org/dist/index.json
#[tokio::main]
async fn get_index_json() -> Result<Vec<Version_info>, Box<dyn Error>> {
    let res = reqwest::get("https://nodejs.org/dist/index.json")
    .await?
    .json::<Vec<Version_info>>()
    .await?;
    Ok(res)
}

pub fn install(config: Config) -> Result<(), Box<dyn Error>> {
    // 第二个参数 版本号
    let version = match config.param2 {
        Some(v) => v,
        None => return Err("版本号不存在".into())
    };
    if version.as_str() < "4" {
        return Err("支持nodejs最低版本为4.0.0".into())
    }
    let version_number = Version_number::parse(version)?;

    let version_infos = get_index_json()?;
    Ok(())
}

// remove
pub fn uninstall(config: Config) {

}

// list
pub fn ls(config: Config) {
    // https://nodejs.org/dist/index.json
}

pub fn _use(config: Config) {

}

// v
pub fn version(config: Config) {

}


// nvm node_mirror [url]        : 设置node镜像。默认值为 https://nodejs.org/dist/。将 [url] 留空以使用默认网址。
// nvm npm_mirror [url]         : 设置 npm 镜像。默认值为 https://github.com/npm/cli/archive/。将 [url] 留空为默认网址。