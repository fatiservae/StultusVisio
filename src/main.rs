//    This file is part of StultusVisio.
//
//    StultusVisio is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    StultusVisio is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with StultusVisio.  If not, see <https://www.gnu.org/licenses/>6.
//    Jefferson T. @ 2023. Telegram: StalinCCCP
use std::{
    env,
    fs::{File, write},
    io::{BufRead, BufReader}
};
use lib::*;
use lib::Handle as H;
mod script;
mod style;
use crate::{
    style::*, 
    script::*
};

/// Um compilador markup para HTML.
///
/// Use `lib::Handle` como token de operações para síntese de HTML válido.
///
/// O processamento do arquivo de entrada é feito linha-a-linha ao estilo 
/// crawler, incrementando a variável `body`.
///
/// O documento `.html` final é concatenado ao fim da compilação.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<String> = env::args().collect();
    for file in files.into_iter().skip(1){
        let reader = BufReader::new(File::open(file.clone())?);
 
        let mut handle: Option<Handle> = None;
        let mut body = String::new();
        let mut title = String::new();
        let mut foot = String::new();
        let mut script_path = None;
        let mut css_path = None;
        let mut logo_path = None;
        let mut script_mermaid = None;
        let mut line_no = 0;

        for line in reader.lines() {
            line_no = line_no + 1;
            let line = line.expect(&format!("Erro na leitura da linha {}", line_no));
            
            if line.starts_with("---") {
                body.push_str(close_last_handle(&handle));
                match handle {
                    None => body.push_str("<!------------------------>\n<div class=\"slide\">"),
                    _ => body.push_str("</div><!------------------------>\n<div class=\"slide\">"),
                }
                handle = None;

            } else if line.is_empty() | line.starts_with("#") {
                continue

            } else if line.starts_with(".image"){
                body.push_str(close_last_handle(&handle));

                let input = file_base64(trim_element(&line), "image")
                            .expect(&format!("Arquivo apontado na linha {} não encontrado.", line_no));

                body.push_str(&format!("<figure><img src=\"{}\">", input));
                handle = Some(H::Image);

            } else if line.starts_with(".caption"){
                let input = trim_element(&line);
                match handle {
                    Some(H::Image) => { 
                        body.push_str(&format!("<figcaption>{}</figcaption>", input)); 
                    },
                    _ => {
                        body.push_str(close_last_handle(&handle));
                    },
                };
                handle = Some(H::Caption);

            } else if line.starts_with(".video"){
                body.push_str(close_last_handle(&handle));
                let input = file_base64(trim_element(&line), "video")?; 
                body.push_str(&format!("<video controls src=\"{}\">", input));
                handle = Some(H::Video);

            } else if line.starts_with(".urlvideo"){
                body.push_str(close_last_handle(&handle));
                let input = trim_element(&line); 
                body.push_str(&format!(
                 "<div class=\"diviframe\"><iframe src=\"{}\" frameborder=\"0\" allowfullscreen=\"true\" >", 
                 input)
                );
                handle = Some(H::URLVideo);

            } else if line.starts_with(".list"){
                body.push_str(close_last_handle(&handle));
                body.push_str(&format!("<ul>"));
                handle = Some(H::List);

            } else if line.starts_with(".ordlist"){
                body.push_str(close_last_handle(&handle));
                body.push_str(&format!("<ol>"));
                handle = Some(H::OrdList);

            } else if line.starts_with(".heading"){
                body.push_str(close_last_handle(&handle));
                let input = trim_element(&line);
                body.push_str(&format!("<h1>{}", input));
                handle = Some(H::Heading);

            } else if line.starts_with(".subheading"){
                body.push_str(close_last_handle(&handle));
                body.push_str(&format!("<h2>{}", trim_element(&line)));
                handle = Some(H::SubHeading);

            } else if line.starts_with(".foot"){
                foot = trim_element(&line);

            } else if line.starts_with(".title"){
                title = format!("{}", trim_element(&line));

            } else if line.starts_with(".text"){
                body.push_str(close_last_handle(&handle));
                handle = Some(H::Text);

            } else if line.starts_with(".script"){
                script_path = Some(trim_element(&line));

            } else if line.starts_with(".css"){
                css_path = Some(trim_element(&line));

            } else if line.starts_with(".mermaid"){
                body.push_str(close_last_handle(&handle));
                body.push_str(&format!("<div class=\"center\"> <pre class=\"mermaid\">"));
                handle = Some(H::Mermaid);

            } else if line.starts_with(".logo"){
                logo_path = Some(trim_element(&line));

            } else if line.starts_with(".frommermaid"){
                script_mermaid = Some(trim_element(&line));

            } else {
                match handle {
                    Some(H::Mermaid) => body.push_str(&format!("{}\n", line)),
                    Some(H::List) => body.push_str(&format!("<li>{}</li>", line)),
                    Some(H::OrdList) => body.push_str(&format!("<li>{}</li>", line)),
                    Some(H::Text) => body.push_str(&format!("<p>{}</p>", line)),
                    _ => body.push_str(&format!("ERROR: verifique a sintaxe deste texto: {}", line)),
                }
        }

    }
        
        let head = format!(
            "<!DOCTYPE html>
            <html lang=\"en\"> 
              <head> 
                <meta charset=\"UTF-8\"> <title>{}</title> 
                {} 
              </head> 
              <body> 
               <div id=\"marcador\"></div> 
               <div id=\"popup\"> <p><span id=\"conteudo-popup\"></span></p> </div>",
            title,
            generate_style(css_path)
        );

        body.push_str(
            &format!(" </div><footer><p>{}</p></footer> {} </body> {} {} </html>", 
             foot, 
             generate_logo(logo_path),
             generate_mermaid_script(script_mermaid),
             generate_script(script_path)
            )
        );

        write(
            stv_to_html(&file), 
            format!("{}{}", head, body)
        )?;
    }

    Ok(())
}
