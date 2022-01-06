mod generator_code;
use anyhow::Result;


fn main() -> Result<()> {
    let font_path = r#"F:\ttf\font-awesome-4.7.0\fonts\fontawesome.ttf"#;
    let font_path = r#"F:\ttf\fontawesome-free-5.15.4-web\webfonts\fa-regular-400.ttf"#;
    let ret = generator_code::generate(font_path)?;
    if ret {
        println!("success!");
    }else{
        println!("fail!");
    }
    Ok(())
}

