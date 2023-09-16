////    This file is part of StultusVisio.
////
////    StultusVisio is free software: you can redistribute it and/or modify
////    it under the terms of the GNU General Public License as published by
////    the Free Software Foundation, either version 3 of the License, or
////    (at your option) any later version.
////
////    StultusVisio is distributed in the hope that it will be useful,
////    but WITHOUT ANY WARRANTY; without even the implied warranty of
////    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
////    GNU General Public License for more details.
////
////    You should have received a copy of the GNU General Public License
////    along with StultusVisio.  If not, see <https://www.gnu.org/licenses/>6.
////    Jefferson T. @ 2023. Telegram: StalinCCCP
//use std::{
//    env,
//    fs::{File, write},
//    io::{BufRead, BufReader}
//};
//use lib::*;
//use lib::Handle as H;
//mod script;
//mod style;
//use crate::{
//    style::*, 
//    script::*
//};
//
///// Um compilador markup para HTML.
/////
///// Use `lib::Handle` como token de operações para síntese de HTML válido.
/////
///// O processamento do arquivo de entrada é feito linha-a-linha ao estilo 
///// crawler, incrementando a variável `body`.
/////
///// O documento `.html` final é concatenado ao fim da compilação.
//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let files: Vec<String> = env::args().collect();
//    for file in files.into_iter().skip(1){
//        let reader = BufReader::new(File::open(file.clone())?);
// 
//       /* let mut handle: Option<Handle> = None;
//        let mut body = String::new();
//        let mut title = String::new();
//        let mut foot = String::new();
//        let mut script_path = None;
//        let mut css_path = None;
//        let mut logo_path = None;
//        let mut script_mermaid = None;
//        let mut line_no = 0;*/
//        let crawler = lib::Crawler {
//            handle: None,
//            title: String::new(),
//            foot: String::new(),
//            script_path: None,  
//            css_path: None, 
//            logo_path: None,  
//            script_mermaid: None,
//            line_no: 0        
//        }
//
//        for line in reader.lines() {
//            
//            let line = line.expect(&format!("Erro na leitura da linha {}", line_no));
//            line.craw(crawler);
//
//        }
//       
//        let head = format!(
//            "<!DOCTYPE html>
//            <html lang=\"en\"> 
//              <head> 
//                <meta charset=\"UTF-8\"> <title>{}</title> 
//                {} 
//              </head> 
//              <body> 
//               <div id=\"marcador\"></div> 
//               <div id=\"popup\"> <p><span id=\"conteudo-popup\"></span></p> </div>",
//            crawler.title,
//            generate_style(crawler.css_path)
//        );
//
//        crawler.body.push_str(
//            &format!(" </div><footer><p>{}</p></footer> {} </body> {} {} </html>", 
//             crawler.foot, 
//             generate_logo(crawler.logo_path),
//             generate_mermaid_script(crawler.script_mermaid),
//             generate_script(crawler.script_path)
//            )
//        );
//
//        write(
//            stv_to_html(&file), 
//            format!("{}{}", head, crawler.body)
//        )?;
//    }
//
//    Ok(())
//}
//
use std::{
    env,
    fs::{File, write},
    io::{BufRead, BufReader},
    error::Error
};
//use lib::*;
//use lib::Handle as H;
//mod script;
//mod style;
//use crate::{
//    style::*,
//    script::*
//};

pub enum Handle {
    Caption,
    Image,
    Video,
    URLVideo,
    Text,
    Heading,
    SubHeading,
    List,
    OrdList,
    Mermaid,
}

struct Presentation {
    handle: Option<Handle>,
    header: String,
    body: String,
    footer: Option<String>,
    title: Option<String>,
    script_path: Option<String>,   //idealmente, Option<Path>
    css_path: Option<String>,
    logo_path: Option<String>,
    script_mermaid: Option<String>,
    line_no: usize
}

trait Process {
    //fn process(&mut self, line: Result<String, dyn Error>) -> Result<(), dyn Error>;
    fn process(&mut self, line: String);
}

impl Process for Presentation {
    //fn process(&mut self, line: Result<String, dyn Error>) -> Result<(), dyn Error>{
    fn process(&mut self, line: String) {
        todo!();
        //match line {
        //    String => {
        //        self.body = line;
        //        self.line_no = self.line_no + 1;
        //        Ok(())
        //    },
        //    Error => Error
        //}
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let files: Vec<String> = env::args().collect();

    for file in files.into_iter().skip(1){
        let mut presentation = Presentation {
                body: String::new(),
                handle: None,
                header: String::new(),
                title: Some(String::new()),
                footer: Some(String::new()),
                script_path: None,
                css_path: None,
                logo_path: None,
                script_mermaid: None,
                line_no: 0
        };

        BufReader::new(File::open(file.clone())?).lines().for_each(|line|
            match line {
                Ok(line) => presentation.process(line),
                //_ => presentation.process(String::from("ERROR"))
                Err(Error) => presentation.process(String::from("ERROR"))
            }
        );

        write(
            //stv_to_html(&file),
            &file,
            format!("{}{}{}", presentation.header, presentation.body, presentation.footer.unwrap())
        )?;
    }
    Ok(())
}
