//    Jefferson T. @ 2023. Telegram: StalinCCCP
//    Licença no fim. Licence at the end.

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
#[derive(Clone, Copy)]
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
    // precisa de controle de no. de linha e lidar com o erro
    input[input.find(' ').expect("Alguma âncora com argumentos falhou!") + 1..].to_string()  
    //if let Some(index) = input.find(' ') {
    //    input[index + 1..].to_string()
    //} else {
    //    input.to_string() //isso deve ser um erro!
    //}
}

/// Fecha o último handle para que a tag correspondente seja propriamente encerrada.
pub fn close_last_handle(handle: Option<Handle>) -> &'static str{
    match handle {
        None => "",
        Some(Handle::Image) => "</figure></div>",
        Some(Handle::Mermaid) => "</pre></div>",
        Some(Handle::Caption) => "</figure></div>",
        Some(Handle::List) => "</ul></div>",
        Some(Handle::OrdList) => "</ol></div>",
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

/// Fornece um script padrão para as âncoras `.mermaid` e analisa o apontamento 
/// feito por `.frommermaid`.
/// O script original mermaid foi criado em fevereiro de 2024, mas pode ser 
/// apontado pela tag `.frommermaid` para versões atualizadas, conforme orientações 
/// de https://mermaid.js.org/.
pub fn generate_mermaid_script(mermaid_script: Option<String>) -> String {
    match mermaid_script {
        Some(mermaid_script) => format!(
                "<script type=\"module\">import mermaid from \'{}\';</script>", 
                mermaid_script),
        None => {
            let mermaid_file = String::from_utf8(include_bytes!("../Config/mermaid-00886c59.js")
                .to_vec()).expect("Não foi possível integrar o script mermaid");
            format!("<script type=\"module\">{};Tt.initialize({{ startOnLoad: true }});</script>", 
                mermaid_file)
        }
    }
}

/// Aponta uma logo para ser renderizada caso exista. 
/// Existindo, será codificada para base64 e incorporada no HTML final.
///
/// A logo é retornada como um elemento fixo da apresentação e, portanto, 
/// o modo printável a retorna apenas no primeiro slide. Porém, este comportamento 
/// é desejável porque StultusVisio é projetado para montar uma apresentação 
/// em HTML e não um arquivo printável. 
pub fn generate_logo(logo_path: Option<String>) -> String {
    match logo_path {
        Some(logo_path) => {
            match file_base64(logo_path, "image") {
                Ok(image64) => image64,
                _ => "falhou ao converter a logo para base64".to_string()
            }
        },
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
            &format!("</div>{} <img src=\"{}\" class=\"logo\" /> </body> {} </html>", 
                match self.footer {
                    Some(footer) => format!("</div><footer><p>{}</p></footer>", footer),
                    None => "".to_string()},
                generate_logo(self.logo_path),
                generate_script(self.script_path)
            )
        );

        write(
            stv_to_html(&file),
            format!(
            r#"
            <!DOCTYPE html>
            <html lang="en"> 
            <!-- Fira Sans -->
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css"> 
            <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
            <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js" onload="renderMathInElement(document.body);"></script>
            <style>
            .katex{{
              text-align: start !important ;
            }}
            </style>
            <link rel="preconnect" href="https://fonts.googleapis.com">
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
            <link href="https://fonts.googleapis.com/css2?family=Fira+Sans&display=swap" rel="stylesheet">
              <head> 
                {} 
                <meta name="description" content="slideshow">
                <meta name="keywords" content="slideshow">
                <meta charset="UTF-8"> <title>{}</title> 
                {}
              </head> 
              <body> 
               <div id="marcador"></div> 
               <div id="popup"> <p><span id="conteudo-popup"></span></p> 
               </div>
            {}"#,
            generate_mermaid_script(self.script_mermaid),
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
            self.body.push_str(close_last_handle(self.handle));
            match self.handle {
                None => self.body.push_str("<!------------------------>\n<div class=\"slide\">"),
                _ => self.body.push_str("</div><!------------------------>\n<div class=\"slide\">"),
            }
            self.handle = None;

        } else if line.starts_with(".image"){
            let input = file_base64(trim_element(&line), "image")
                .expect(&format!("Arquivo apontado na linha {} não encontrado.", line_no));
            match self.handle {
                Some(Handle::Image) => {
                    self.body.push_str(&format!("</figure><figure><img src=\"{}\">", input));
                },
                Some(Handle::Caption) => {
                    self.body.push_str(&format!("</figure><figure><img src=\"{}\">", input));
                },//fechar so o figure,
                _ => {
                    self.body.push_str(close_last_handle(self.handle));
                    self.body.push_str(&format!("<div class=\"images\"><figure><img src=\"{}\">", input));
                }
            }
            self.handle = Some(Handle::Image);

        } else if line.starts_with(".caption"){
            let input = trim_element(&line);
            match self.handle {
                Some(Handle::Image) => { 
                    self.body.push_str(&format!("<figcaption>{}</figcaption>", input)); 
                },
                // pode ser aberto para outro Handle etc.
                _ => {
                    self.body.push_str(close_last_handle(self.handle));
                },
            };
            self.handle = Some(Handle::Caption);

        } else if line.starts_with(".video"){
            self.body.push_str(close_last_handle(self.handle));
            let input = file_base64(trim_element(&line), "video")?; 
            self.body.push_str(&format!("<video controls src=\"{}\">", input));
            self.handle = Some(Handle::Video);

        } else if line.starts_with(".urlvideo"){
            self.body.push_str(close_last_handle(self.handle));
            let input = trim_element(&line); 
            self.body.push_str(&format!(
             "<div class=\"diviframe\"><iframe src=\"{}\" frameborder=\"0\" allowfullscreen=\"true\" >", 
             input)
            );
            self.handle = Some(Handle::URLVideo);

        // As listas vão depender de um pré-compilador
        // que aglutine as âncoras .list e/ou .ordlist
        } else if line.starts_with(".list"){
            match self.handle {
                Some(Handle::List) => {
                    self.body.push_str(&format!("</ul>"));
                    self.body.push_str(&format!("<ul>"));
                },
                Some(Handle::OrdList) => {
                    self.body.push_str(&format!("</ol>"));
                    self.body.push_str(&format!("<ul>"));
                },
                _ => {
                    self.body.push_str(close_last_handle(self.handle));
                    self.body.push_str(&format!("<div class=\"listas\"><ul>"));
                    self.handle = Some(Handle::List);
                }
            };
            
        } else if line.starts_with(".ordlist"){
            match self.handle {
                Some(Handle::List) => {
                    self.body.push_str(&format!("</ul>"));
                    self.body.push_str(&format!("<ol>"));
                },
                Some(Handle::OrdList) => {
                    self.body.push_str(&format!("</ol>"));
                    self.body.push_str(&format!("<ol>"));
                },
                _ => {
                    self.body.push_str(close_last_handle(self.handle));
                    self.body.push_str(&format!("<div class=\"listas\"><ol>"));
                    self.handle = Some(Handle::OrdList);
                }
            };
 
        } else if line.starts_with(".heading"){
            self.body.push_str(close_last_handle(self.handle));
            let input = trim_element(&line);
            self.body.push_str(&format!("<h1>{}", input));
            self.handle = Some(Handle::Heading);

        } else if line.starts_with(".subheading"){
            self.body.push_str(close_last_handle(self.handle));
            self.body.push_str(&format!("<h2>{}", trim_element(&line)));
            self.handle = Some(Handle::SubHeading);

        } else if line.starts_with(".foot"){
            self.footer = Some(trim_element(&line));
            // incluir todo format do footer!

        } else if line.starts_with(".title"){
            self.title = Some(format!("{}", trim_element(&line)));
            // incluir todo format do title

        } else if line.starts_with(".text"){
            self.body.push_str(close_last_handle(self.handle));
            self.handle = Some(Handle::Text);

        } else if line.starts_with(".script"){
            self.script_path = Some(trim_element(&line));

        } else if line.starts_with(".css"){
            self.css_path = Some(trim_element(&line));

        } else if line.starts_with(".mermaid"){
            self.body.push_str(close_last_handle(self.handle));
            self.body.push_str(&format!("<div class=\"center\"> <pre class=\"mermaid\">"));
            self.handle = Some(Handle::Mermaid);

        } else if line.starts_with(".frommermaid"){
            self.script_mermaid = Some(trim_element(&line));

        } else if line.starts_with(".logo"){
            self.logo_path = Some(trim_element(&line));

        } else if line.starts_with(".table"){
            self.body.push_str(close_last_handle(self.handle));
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

//    LICENCE
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

