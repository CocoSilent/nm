use std::{error::Error, path::Path, fs, io::{Write, self}, process::Command, str::FromStr};
use serde::{Deserialize};

use crate::Config;

#[derive(Debug, Deserialize)]
// https://nodejs.org/dist/index.json返回的结构
struct Version_info {
    version: String,
}

// 版本号
#[derive(Debug)]
struct Version_number {
    one: String,
    two: String,
    three: String,
    version: String,
    len: usize,
}

impl Version_number {
    fn parse<'a>(version: String) -> Result<Version_number, String> {
        let arr: Vec<_> = version.split('.').collect();
        if arr.len() > 3 {
            let str = format!("版本号：{version},格式不正确");
            return Err(str);
        }
        if arr[0] < "4" {
            return Err("支持nodejs最低版本为4.0.0".into())
        }
        Ok(Version_number { 
            one: arr[0].to_string(),
            two: arr[1].to_string(),
            three: arr[2].to_string(),
            len: arr.len(),
            version: version,
        })
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

// 下载文件
#[tokio::main]
async fn download(url: &String, file_name: &String) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(url)
    .await?
    .bytes()
    .await?;

    let path = Path::new(file_name);
    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    file.write_all(&res)?;

    Ok(())
}

fn unzip(zipName: &String) -> i32 {
    let fname = std::path::Path::new(zipName);
    let file = fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut root_path = String::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        if i==0 {
            root_path = outpath.clone().into_os_string().into_string().unwrap();
        }

        // {
        //     // 文件注释
        //     let comment = file.comment();
        //     if !comment.is_empty() {
        //         println!("File {i} comment: {comment}");
        //     }
        // }

        if (*file.name()).ends_with('/') {
            // println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // println!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            // );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    // 去掉后缀
    let new_path = zipName.replace(".zip", "");
    if &root_path != "" {
        fs::rename(root_path, new_path).unwrap();
    }
    
    0
}

pub fn install(config: Config) -> Result<(), Box<dyn Error>> {
    // 第二个参数 版本号
    let version = match config.param2 {
        Some(v) => v,
        None => return Err("版本号不存在".into())
    };
    let version_number = Version_number::parse(version)?;
    println!("{:?}",version_number);
    let version_infos = get_index_json()?;
    // todo 版本号 1 2 3位分别匹配， 暂时先按整体来算了
    let v = format!("v{}", version_number.version);
    for version_info in version_infos {
        if version_info.version.starts_with(&v) {
            println!("{}", &version_number.version);
            break;
        }
    }
    // 下载
    let file_name = format!("{v}.zip");
    let url = format!("https://nodejs.org/dist/{v}/node-{v}-win-x64.zip");
    download(&url, &file_name)?;
    unzip(&file_name);
    println!("安装完成：{v}");
    // 解压
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
    let version = match config.param2 {
        Some(v) => v,
        None => return (),
    };
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "mklink", "/J", "nodejs", "v9.6.1"])
                .output()
                .expect("创建目录链接失败")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    println!("status: {}", output.status);
    println!("nm use的结果是{}", String::from_utf8_lossy(&output.stdout));
}

// v
pub fn version(config: Config) {

}


// nvm node_mirror [url]        : 设置node镜像。默认值为 https://nodejs.org/dist/。将 [url] 留空以使用默认网址。
// nvm npm_mirror [url]         : 设置 npm 镜像。默认值为 https://github.com/npm/cli/archive/。将 [url] 留空为默认网址。