use std::error::Error;

use crate::Config;

// https://nodejs.org/dist/index.json
#[tokio::get_index_json]
async fn get_index_json() -> Result<(), Box<dyn, Error>> {
    let res = reqwest::get("https://nodejs.org/dist/index.json")
    .await?;

    println!("{res}");
}

pub fn install(config: Config) {
    get_index_json();
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