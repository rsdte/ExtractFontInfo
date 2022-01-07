mod generator_code;

use std::fs::File;
use std::path::Path;
use std::env;
use anyhow::{anyhow, Result};
use crate::generator_code::generate;
use clap::{AppSettings, Parser};

/// 给定字体文件路径，生成对应的 c# 代码。
#[derive(Parser, Debug)]
struct GenerateCode {
    /// 字体文件地址/路径
    #[clap(short, long)]
    font_path: String,
    /// 代码存放路径, 是目录。默认为当前目录。
    #[clap(short, long, default_value_t = String::new())]
    output_path: String,
    /// 输出文件名称，也为枚举名称，默认为字体文件名称
    #[clap(short, long, default_value_t = String::new())]
    enum_name: String
}

fn main() -> Result<()> {
    let mut opts: GenerateCode = GenerateCode::parse();
    // let mut opts = GenerateCode {
    //     output_path: String::new(),
    //     enum_name: String::new(),
    //     font_path: r#"F:\ttf\font-awesome-4.7.0\fonts\fontawesome.ttf"#.to_string()
    // };
    if !opts.font_path.is_empty() {
        let font_path = Path::new(opts.font_path.as_str());
        let font_file_name = font_path.file_name().unwrap().to_str().unwrap();
        if opts.output_path.is_empty() {
            let path = env::current_dir().unwrap();
            opts.output_path = String::from(path.to_str().unwrap());
        }
        if opts.enum_name.is_empty() {
            let name = font_file_name.split('.').nth(0).unwrap();
            opts.enum_name = String::from(name);
        }
        generator_code::generate(opts.font_path.as_str(), opts.output_path.as_str(), opts.enum_name)?;
    }
    Ok(())
}

