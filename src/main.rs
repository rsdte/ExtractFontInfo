use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use anyhow::Result;
use opentype::{Font, GlyphDefinition};
use opentype::postscript::compact1::FontSet;
use opentype::postscript::Tape;
use opentype::truetype::{CharMapping, GlyphData, GlyphMapping, NamingTable, PostScript};
use opentype::truetype::char_mapping::Encoding;


fn main() -> Result<()> {
    let font_path = r#"F:\ttf\font-awesome-4.7.0\fonts\fontawesome.ttf"#;
    let font_path = r#"F:\ttf\fontawesome-free-5.15.4-web\webfonts\fa-regular-400.ttf"#;
    let mut reader = File::open(font_path)?;
    let font = Font::read(&mut reader)?;
    let data: PostScript = font.take(&mut reader)?.unwrap();
    let mut names = Vec::new();
    match data
    {
        PostScript::Version2(post2) => {
            println!("{:?}", post2.glyph_names);
            names = post2.glyph_names;
        }
        _ => println!("None.")
    }

    let csPath = Path::new(r#"F:\ttf\FontAwesomeIcon.cs"#);
    let mut file = File::create(csPath)?;

    file.write("public enum FontAwesomeIcon {\r\n".as_bytes());


    let data: CharMapping = font.take(&mut reader)?.unwrap();
    for v in data.encodings {
        match v {
            Encoding::Format4(ref enc) => {
                let map = enc.mapping();
                for m in map {
                    let index = m.1 as usize;
                    if index < names.len() {
                        let name = &names[index];
                        if !name.starts_with("_") || !name.starts_with("-") {
                            let split = if name.contains("-") {
                                '-'
                            } else {
                                '_'
                            };
                            let k = name.split(split).map(|x| to_first_upper(x).unwrap()).collect::<Vec<String>>();
                            let enum_name = k.join("");
                            file.write(format!(r#"    [Description("{0}"),IconId("{1}")]"#, enum_name, name).as_bytes());
                            file.write(format!("\r\n    {0} = {1:#x},\r\n", enum_name, m.0).as_bytes());
                        } else {
                            file.write(format!(r#"    [Description("{0}"),IconId("{1}")]"#, name, name).as_bytes());
                            file.write(format!("\r\n    {0} = {1:#x},\r\n", name, m.0).as_bytes());
                        }
                    }
                }
            }
            _ => {
                println!("没有数据。");
            }
        }
    }
    file.write("}".as_bytes());
    file.flush()?;
    Ok(())
}


fn main1() {
    let mut s = String::from("hello");
    println!("{:?}", to_first_upper(s.as_str()));
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
