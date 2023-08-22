use std::fs::File;
use std::io::{BufRead, BufReader};
use lib::*;
use lib::Handle::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("presentation.stv")?;
    let reader = BufReader::new(file);
 
    let mut handle: Option<Handle> = None;
    let mut body = String::new();
    let mut title = String::new();
    let mut foot = String::new();
    let mut script_path = None;
    let mut css_path = None;
    let mut logo_path = None;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("---") {
            body.push_str(close_last_handle(&handle));
            match handle {
                None => body.push_str("<!------------------------>\n<div class=\"page\">"),
                _ => body.push_str("</div><!------------------------>\n<div class=\"page\">"),
            }
            handle = None;

        } else if line.is_empty() | line.starts_with("#") {
            continue

        } else if line.starts_with(".image"){
            body.push_str(close_last_handle(&handle));
            let input = file_base64(trim_element(&line))?;
            body.push_str(&format!("<figure><img src=\"data:image;base64,{}\">", input));
            handle = Some(Image);

        } else if line.starts_with(".caption"){
            let input = trim_element(&line);
            match handle {
                Some(Image) => { 
                    body.push_str(&format!("<figcaption>{}</figcaption>", input)); 
                },
                _ => {
                    body.push_str(close_last_handle(&handle));
                },
            };
            handle = Some(Caption);

        } else if line.starts_with(".video"){
            body.push_str(close_last_handle(&handle));
            let input = file_base64(trim_element(&line))?; 
            body.push_str(&format!("<video controls src=\"data:video;base64,{}\"", input));
            handle = Some(Video);

        } else if line.starts_with(".list"){
            body.push_str(close_last_handle(&handle));
            body.push_str(&format!("<ul>"));
            handle = Some(List);

        } else if line.starts_with(".ordlist"){
            body.push_str(close_last_handle(&handle));
            body.push_str(&format!("<ol>"));
            handle = Some(OrdList);

        } else if line.starts_with(".heading"){
            body.push_str(close_last_handle(&handle));
            let input = trim_element(&line);
            body.push_str(&format!("<h1>{}", input));
            handle = Some(Heading);

        } else if line.starts_with(".subheading"){
            body.push_str(close_last_handle(&handle));
            body.push_str(&format!("<h2>{}", trim_element(&line)));
            handle = Some(SubHeading);

        } else if line.starts_with(".foot"){
            foot = trim_element(&line);

        } else if line.starts_with(".title"){
            title = format!("{}", trim_element(&line));

        } else if line.starts_with(".text"){
            body.push_str(close_last_handle(&handle));
            handle = Some(Text);

        } else if line.starts_with(".script"){
            script_path = Some(trim_element(&line));

        } else if line.starts_with(".css"){
            css_path = Some(trim_element(&line));

        } else if line.starts_with(".logo"){
            logo_path = Some(trim_element(&line));

        } else {
            match handle {
                Some(List) => body.push_str(&format!("<li>{}</li>", line)),
                Some(OrdList) => body.push_str(&format!("<li>{}</li>", line)),
                Some(Text) => body.push_str(&format!("<p>{}</p>", line)),
                _ => body.push_str(&format!("ERROR: verifique a sintaxe deste texto: {}", line)),
            }
        }
     }
    
    let head = format!(
        "<html lang=\"en\"> <head> <meta charset=\"UTF-8\"> <title>{}</title> {} </head> ",
        title,
        generate_logo(logo_path),
    );

    body.push_str(
        &format!("<body> <div id=\"marcador\"></div> <div id=\"popup\"> <p><span id=\"conteudo-popup\"></span></p> </div> {} <!------------------------> <footer> <p>{}</p> </footer></body> {} </html>", 
         generate_style(css_path),
         foot, 
         generate_script(script_path)
        )
    );

    println!("{}\n{}", head, body);
    Ok(())
}
