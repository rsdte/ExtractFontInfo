use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use anyhow::Result;
use opentype::Font;
use opentype::truetype::{CharMapping, PostScript};
use opentype::truetype::char_mapping::Encoding;
use regex::Regex;


pub fn generate(font_path: &str, output_path: &str, file_name: String) -> Result<bool> {
    let path = Path::new(font_path);
    let mut reader = File::open(path)?;
    let font = Font::read(&mut reader)?;
    let data: PostScript = font.take(&mut reader)?.unwrap();
    let mut names = Vec::new();

    if let PostScript::Version2(post2) = data {
        names = post2.glyph_names;
    }else{
        return Ok(false);
    }

    let mut cs_path = PathBuf::new();
    cs_path.push(output_path);
    cs_path.push(format!("{0}.cs", file_name.as_str()));
    println!("{:?}", cs_path);
    // let mut file = File::create(cs_path)?;
    let mut file = File::create(cs_path)?;
    file.write(format!(r#"public enum {0} {{{1}"#, file_name, "\r\n").as_bytes())?;
    let data: CharMapping = font.take(&mut reader)?.unwrap();
    let regex = Regex::new(r#"^\d"#)?;
    for i in data.encodings {
        if let Encoding::Format4(ref enc) = i {
            let map = enc.mapping();
            for m in map {
                let index = m.1 as usize;
                if index < names.len() {
                    let name = &names[index];
                    if name.contains("-") || name.contains("_") {
                        // 包含 ’-‘、’_‘ 的名称去掉转驼峰命名
                        let split = if name.contains("-") {
                            '-'
                        } else {
                            '_'
                        };
                        let k = name.split(split).map(|x| to_first_upper(x).unwrap()).collect::<Vec<String>>();
                        let mut enum_name = k.join("");
                        if regex.is_match(enum_name.as_str()){
                            // 如果是数字开头就加上 '_'
                            enum_name.insert(0, '_');
                        }
                        file.write(format!(r#"    [Description("{0}"),IconId("{1}")]"#, enum_name, name).as_bytes())?;
                        file.write(format!("\r\n    {0} = {1:#x},\r\n", enum_name, m.0).as_bytes())?;
                    } else {
                        let mut enum_name = to_first_upper(name.as_str()).unwrap();
                        if regex.is_match(enum_name.as_str()){
                            // 如果是数字开头就加上 '_'
                            enum_name.insert(0, '_');
                        }
                        file.write(format!(r#"    [Description("{0}"),IconId("{1}")]"#, enum_name, name).as_bytes())?;
                        file.write(format!("\r\n    {0} = {1:#x},\r\n", enum_name , m.0).as_bytes())?;
                    }
                }
            }
        }
    }

    file.write("}".as_bytes())?;
    file.flush()?;
    Ok(true)
}

fn to_first_upper(word: &str) -> Result<String> {
    let mut buider = Vec::new();
    for (index, ch) in word.chars().enumerate() {
        if index < 1 {
            let v = ch.to_uppercase().collect::<Vec<char>>();
            buider.push(v[0] as u8);
        } else {
            let v = ch.to_lowercase().collect::<Vec<char>>();
            buider.push(v[0] as u8);
        }
    }
    Ok(String::from_utf8(buider)?)
}
