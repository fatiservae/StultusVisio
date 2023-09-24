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
use{
    std::{
        fs,
        fs::write, 
        path::Path,
    },
    base64,
};
pub mod script;
pub mod style;
use crate::{
    style::*,
    script::*
};

/// Possíveis handles que indicam como a próxima linha será processada.
///
/// Na struct `Presentation` o elemento `handle` recebe a enumeração 
/// correspondente de acordo com a última âncora indicada.
pub enum Handle {
    /// Indica legendas. 
    ///
    /// Necessita operar sobre uma linha prévia que aceita legendas, 
    /// e.g. `Handle::Image`.
    Caption,
    /// Indica elemento `<figure>`.
    Image,
    /// Indica elemento `<video>`.
    Video,
    /// Indica elemento `<video>` construída a partir de URL.
    URLVideo,
    /// TODO.
    Text,
    /// Indica elemento `<h1>`.
    Heading,
    /// Indica elemento `<h2>`.
    SubHeading,
    /// Indica início do elemento `<ul>` e que os próximos elementos a serem lidos 
    /// são `<li>` caso mantenham `Handle::List`.
    List,
    /// Indica início do elemento `<ol>` e que os próximos elementos a serem lidos 
    /// são `<li>` caso mantenham `Handle::OrdList`.
    OrdList,
    /// Indica as próximas linhas como linhas literais de instruções Mermaid.
    ///
    /// [Mermaid](https://mermaid.js.org/)
    Mermaid,
    /// Indica o construtor de `HTML` a partir de texto literal em `Markdown`.
    ///
    /// A API utilziada é [markdown](https://crates.io/crates/markdown) disponível em crates.io.
    Table(usize)
}

/// Remove a âncora da linha que a contém.
pub fn trim_element(input: &String) -> String {
    if let Some(index) = input.find(' ') {
        let cut_string = input[index + 1..].to_string();
        cut_string
    } else {
        input.to_string() //isso deve ser um erro!
    }
}

/// Fecha o último handle para que a tag correspondente seja propriamente encerrada.
pub fn close_last_handle(handle: &Option<Handle>) -> &'static str{
    match handle {
        None => "",
        Some(Handle::Image) => "</figure>",
        Some(Handle::Mermaid) => "</pre></div>",
        Some(Handle::Caption) => "</figure>",
        Some(Handle::List) => "</ul>",
        Some(Handle::OrdList) => "</ol>",
        Some(Handle::Text) => "</p>",
        Some(Handle::Heading) => "</h1>",
        Some(Handle::SubHeading) => "</h2>",
        Some(Handle::Video) => "</video>",
        Some(Handle::URLVideo) => "</iframe></div>",
        Some(Handle::Table(_x)) => "</table>" 
    }
}

/// Converte arquivo apontado para base64 e incorpora o `raw data` no HTML final.
pub fn file_base64(file: String, tipo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_data = fs::read(file.clone())
                        .expect("Arquivo de mídia não encontrado para converter em base64");

    Ok(format!("data:{}/{};base64,{}", 
            tipo, 
            Path::new(&file.clone())
                        .extension()
                        .expect(&format!("Erro ao determinar o tipo de arquivo de: {}", &file))
                        .to_str()
                        .ok_or(&format!("Erro ao converter o caminho de {} para string.", &file))
                        .expect(&format!("Erro ao validar {} como caminho de arquivo", &file)),
            base64::encode(&file_data)
    ))
}

/// Fornece um script padrão para as âncoras `.mermaid` ou aceita um apontamento 
/// feito por `.frommermaid`
pub fn generate_mermaid_script(mermaid_script: Option<String>) -> String {
    match mermaid_script {
        Some(mermaid_script) => format!(
                "<script type=\"module\">import mermaid from '{}';</script>", 
                mermaid_script),
        None => "<script type=\"module\">import mermaid from 
            'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
            </script>".to_string()
    }
}

/// Aponta uma logo para ser renderizada caso exista.
pub fn generate_logo(logo_path: Option<String>) -> String {
    match logo_path {
        Some(logo_path) => format!("<img src=\"{}\" class=\"logo\">", logo_path),
        None => "".to_string(),
    }
}

/// Valida o arquivo de entrada como um stv válido. Atualmente, apenas checka se 
/// o arquivo é um .stv. 
///
/// TODO: adicionar capacidade de verificar sintaxe.
pub fn validate_stv(name: &str) -> Result<bool, ()> {
    if name.ends_with(".stv") {
        Ok(true)
    } else { 
        Err(())
    }
}

/// Corta a extensão `.stv` e apensa `.html` o nome de arquivo que será usado como output.
pub fn stv_to_html(name: &str) -> String {
    format!("{}.html", &name[..&name.len() - 4]).to_string()
}

/// Abstração de uma apresentação.
pub struct Presentation {
    pub handle: Option<Handle>,
    pub header: String,
    pub body: String,
    pub footer: Option<String>,
    pub title: Option<String>,
    pub script_path: Option<String>,   //idealmente, Option<Path>
    pub css_path: Option<String>,
    pub logo_path: Option<String>,
    pub script_mermaid: Option<String>,
}

/// Opera sobre um tipo Presentation confeccionando o `HTLM` correspondente para cada String fornecida. 
/// O argumento `line_no` deve ser usado para identificar linhas onde ocorrem erros durante a confecção
/// do `HTML` correspondente. 
pub trait Process {
    //fn process(&mut self, line: Result<String, dyn Error>) -> Result<(), dyn Error>;
    fn process(&mut self, line: String, line_no: usize) -> Result<(), Box<dyn std::error::Error>>;
}

/// Constrói o arquivo final `file.html` como método do tipo Presentation.
/// O nome do arquivo de entrada `file` será usado para nomear a saída.
pub trait Build {
    fn build(self, file: String) -> Result<(), Box<dyn std::error::Error>>;
}

impl Build for Presentation {
    fn build(mut self, file: String) -> Result<(), Box<dyn std::error::Error>> {
        self.body.push_str(
            &format!("{} {} </body> {} {} </html>", 
             match self.footer {
                 Some(footer) => format!("</div><footer><p>{}</p></footer>", footer),
                 None => "".to_string()},
             generate_logo(self.logo_path),
             generate_mermaid_script(self.script_mermaid),
             generate_script(self.script_path)
        ));

        write(
            stv_to_html(&file),
            format!(
            "<!DOCTYPE html>
            <html lang=\"en\"> 
              <head> 
                <meta charset=\"UTF-8\"> <title>{}</title> 
                {} 
              </head> 
              <body> 
               <div id=\"marcador\"></div> 
               <div id=\"popup\"> <p><span id=\"conteudo-popup\"></span></p> 
               </div>
            {}",
            match self.title {Some(title) => title, None => "".to_string()},
            generate_style(self.css_path),
            self.body)
        )?;
        Ok(())
    }
}

impl Process for Presentation {
    /// Método que opera sobre `Presentation` e recebe String como argumento e 
    /// sintetiza o `HTML` correspondente.
    ///
    /// O argumento `line_no` indica número de linha para correto apontamento
    /// de erros.
    fn process(&mut self, line: String, line_no: usize) -> Result<(), Box<dyn std::error::Error>> {
        if line.is_empty() | line.starts_with("#") | (line.starts_with("---")&&line.len()>3){
            // nada
        }

        else if line.starts_with("---") { // considerar restringir se line.len() == 3 
            self.body.push_str(close_last_handle(&self.handle));
            match self.handle {
                None => self.body.push_str("<!------------------------>\n<div class=\"slide\">"),
                _ => self.body.push_str("</div><!------------------------>\n<div class=\"slide\">"),
            }
            self.handle = None;

        } else if line.starts_with(".image"){
            self.body.push_str(close_last_handle(&self.handle));

            let input = file_base64(trim_element(&line), "image")
                        .expect(&format!("Arquivo apontado na linha {} não encontrado.", line_no));

            self.body.push_str(&format!("<figure><img src=\"{}\">", input));
            self.handle = Some(Handle::Image);

        } else if line.starts_with(".caption"){
            let input = trim_element(&line);
            match self.handle {
                Some(Handle::Image) => { 
                    self.body.push_str(&format!("<figcaption>{}</figcaption>", input)); 
                },
                // pode ser aberto para outro Handle etc.
                _ => {
                    self.body.push_str(close_last_handle(&self.handle));
                },
            };
            self.handle = Some(Handle::Caption);

        } else if line.starts_with(".video"){
            self.body.push_str(close_last_handle(&self.handle));
            let input = file_base64(trim_element(&line), "video")?; 
            self.body.push_str(&format!("<video controls src=\"{}\">", input));
            self.handle = Some(Handle::Video);

        } else if line.starts_with(".urlvideo"){
            self.body.push_str(close_last_handle(&self.handle));
            let input = trim_element(&line); 
            self.body.push_str(&format!(
             "<div class=\"diviframe\"><iframe src=\"{}\" frameborder=\"0\" allowfullscreen=\"true\" >", 
             input)
            );
            self.handle = Some(Handle::URLVideo);

        } else if line.starts_with(".list"){
            self.body.push_str(close_last_handle(&self.handle));
            self.body.push_str(&format!("<ul>"));
            self.handle = Some(Handle::List);

        } else if line.starts_with(".ordlist"){
            self.body.push_str(close_last_handle(&self.handle));
            self.body.push_str(&format!("<ol>"));
            self.handle = Some(Handle::OrdList);

        } else if line.starts_with(".heading"){
            self.body.push_str(close_last_handle(&self.handle));
            let input = trim_element(&line);
            self.body.push_str(&format!("<h1>{}", input));
            self.handle = Some(Handle::Heading);

        } else if line.starts_with(".subheading"){
            self.body.push_str(close_last_handle(&self.handle));
            self.body.push_str(&format!("<h2>{}", trim_element(&line)));
            self.handle = Some(Handle::SubHeading);

        } else if line.starts_with(".foot"){
            self.footer = Some(trim_element(&line));
            // incluir todo format do footer!

        } else if line.starts_with(".title"){
            self.title = Some(format!("{}", trim_element(&line)));
            // incluir todo format do title

        } else if line.starts_with(".text"){
            self.body.push_str(close_last_handle(&self.handle));
            self.handle = Some(Handle::Text);

        } else if line.starts_with(".script"){
            self.script_path = Some(trim_element(&line));

        } else if line.starts_with(".css"){
            self.css_path = Some(trim_element(&line));

        } else if line.starts_with(".mermaid"){
            self.body.push_str(close_last_handle(&self.handle));
            self.body.push_str(&format!("<div class=\"center\"> <pre class=\"mermaid\">"));
            self.handle = Some(Handle::Mermaid);

        } else if line.starts_with(".frommermaid"){
            self.script_mermaid = Some(trim_element(&line));

        } else if line.starts_with(".logo"){
            self.logo_path = Some(trim_element(&line));

        } else if line.starts_with(".table"){
            self.body.push_str(close_last_handle(&self.handle));
            self.handle = Some(Handle::Table(0));

        } else { // se nenhuma âncora existe, tratar o texto
                 // de forma literal de acordo com o respectivo
                 // Handler.
            match self.handle {
                Some(Handle::Mermaid) => self.body.push_str(&format!("{}\n", line)),
                Some(Handle::List) => self.body.push_str(&format!("<li>{}</li>", line)),
                Some(Handle::OrdList) => self.body.push_str(&format!("<li>{}</li>", line)),
                Some(Handle::Text) => self.body.push_str(&format!("<p>{}</p>", line)),
                Some(Handle::Table(mut x)) => {
                    self.body.push_str(&table_generator(&line, x));
                    x = x + 1;
                    self.handle = Some(Handle::Table(x))
                },
                _ => self.body.push_str(&format!("ERROR: verifique a sintaxe deste texto: {}", line)),
            }
        }

        Ok(())
    }
}

/// Processa linhas como `rows` de uma table em `HTML`. 
///
/// O argumento `x: usize` é a carga da variante `Handle::Table(x)`. Para informar
/// o compilador de que a primeira linha se trata do cabeçalho '<th>' da tabela,
/// este valor é iniciado como `0`.  Este valor é incrementado em toda linha de tabela
/// processada, sendo útil na manipulação de erros ou formatação condicional de linhas. 
pub fn table_generator(line: &String, x: usize) -> String {
    match x { 
        // x pode ser condicionado aqui.
        0 => format!("<table> <tr><th>{}</th></tr>",
                    line.replace("|", "</th><th>")),
        _ => format!("<tr><td>{}</td></tr>",
                    line.replace("|", "</td><td>")),
    }
}
