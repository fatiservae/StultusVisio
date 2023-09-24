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
use lib::*;
use std::{
    env,
    fs::File, 
    io::{BufRead, BufReader},
    error::Error
};

fn main() -> Result<(), Box<dyn Error>> {
    let files: Vec<String> = env::args().collect();

    for file in files.into_iter().skip(1){
        let mut presentation = lib::Presentation {
                body: String::new(),
                handle: None,
                header: String::new(),
                title: Some(String::new()),
                footer: Some(String::new()),
                script_path: None,
                css_path: None,
                logo_path: None,
                script_mermaid: None,
        };

        let mut line_no = 0;
        BufReader::new(File::open(file.clone())?).lines().for_each(|line|{
            let _ = match line { // não interessa o resultado, o BufReader falha com erro próprio.
                Ok(line) => presentation.process(line, line_no),
                Err(_error) => todo!() 
            };
            line_no = line_no + 1;
        });

        presentation.build(file)?

    }
    Ok(())
}
