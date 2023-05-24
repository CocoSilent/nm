use std::error::Error;
use serde::{Deserialize};

use crate::Config;

#[derive(Debug, Deserialize)]
struct version_info {
    version: String,
}

// https://nodejs.org/dist/index.json
#[tokio::main]
async fn get_index_json() -> Result<Vec<version_info>, Box<dyn Error>> {
    let res = reqwest::get("https://nodejs.org/dist/index.json")
    .await?
    .json::<Vec<version_info>>()
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
    let version_infos = get_index_json()?;
    Ok(())
}

// remove
pub fn uninstall(config: Config) {

}

// list
pub fn ls(config: Config) {

}

pub fn _use(config: Config) {

}

// v
pub fn version(config: Config) {

}


// nvm node_mirror [url]        : 设置node镜像。默认值为 https://nodejs.org/dist/。将 [url] 留空以使用默认网址。
// nvm npm_mirror [url]         : 设置 npm 镜像。默认值为 https://github.com/npm/cli/archive/。将 [url] 留空为默认网址。