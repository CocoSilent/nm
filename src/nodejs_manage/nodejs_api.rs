use std::{error::Error, path::Path, fs, io::{Write, self}, process::Command};
use serde::{Deserialize};

use crate::Config;

#[derive(Debug, Deserialize)]
// https://nodejs.org/dist/index.json返回的结构
struct VersionInfo {
    version: String,
}

// 版本号
#[derive(Debug)]
struct VersionNumber {
    version: String,
    len: usize,
}

impl VersionNumber {
    fn parse<'a>(version: String) -> Result<VersionNumber, String> {
        let arr: Vec<_> = version.split('.').collect();
        if arr.len() > 3 {
            let str = format!("版本号：{version},格式不正确");
            return Err(str);
        }

        let major = match arr[0].parse::<i32>()  {
            Ok(x) => x,
            Err(_e) => {
                let str = format!("版本号：{version},第一位格式不正确");
                return Err(str)
            },
        };

        if major < 4 {
            return Err("支持nodejs最低版本为4.0.0".into())
        }
        Ok(VersionNumber { 
            len: arr.len(),
            version: version,
        })
    }
}

// https://nodejs.org/dist/index.json
#[tokio::main]
async fn get_index_json() -> Result<Vec<VersionInfo>, Box<dyn Error>> {
    let res = reqwest::get("https://nodejs.org/dist/index.json")
    .await?
    .json::<Vec<VersionInfo>>()
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

fn unzip(zip_name: &String) -> i32 {
    let fname = std::path::Path::new(zip_name);
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
    let new_path = zip_name.replace(".zip", "");
    if &root_path != "" {
        fs::rename(root_path, new_path).unwrap();
    }
    
    0
}

pub fn install(config: Config) -> Result<(), Box<dyn Error>> {
    // 第二个参数 版本号
    let version = match config.param2 {
        Some(v) => v,
        None => return Err("请输入版本号".into())
    };
    let version_number = VersionNumber::parse(version)?;
    let version_infos = get_index_json()?;
    let mut match_v = format!("v{}", version_number.version);
    if version_number.len < 3 {
        match_v = match_v + ".";
    }
    let mut v = String::new();
    for version_info in version_infos {
        if version_info.version.starts_with(&match_v) {
            println!("{}", &version_number.version);
            v = version_info.version;
            break;
        }
    }
    println!("即将下载的版本是:{}", v);
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
                .args(["/C", "mklink", "/J", "nodejs", &("v".to_owned() + &version) ])
                .output()
                .expect("创建目录链接失败")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    // println!("status: {}", output.status);
    let code = output.status.code().unwrap();
    if code == 0 {
        println!("设置成功，当前版本是：{version}")
    } else {
        println!("设置失败")
    }
    // println!("nm use的结果是{}", String::from_utf8_lossy(&output.stdout));
}

// v
pub fn version(config: Config) {
    println!("当前版本是：0.0.1")
}


// nvm node_mirror [url]        : 设置node镜像。默认值为 https://nodejs.org/dist/。将 [url] 留空以使用默认网址。
// nvm npm_mirror [url]         : 设置 npm 镜像。默认值为 https://github.com/npm/cli/archive/。将 [url] 留空为默认网址。