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
        path::Path,
    },
    base64
};

/// Possíveis handles que indicam como a próxima linha será processada.
///
/// TODO: Idealmente, `main()` deveria ser feita idiomaticamente por 
/// métodos que confiram o valor de Handle e ofereçam a confecção adequada 
/// do HTML correspondente ou devolva um erro verboso. Comportamentos 
/// adaptativos também podem ser ofertados. 
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
pub fn close_last_handle(handle: &Option<Handle>) -> &str {
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
    }
}

/// Converte arquivo apontado para base64 e incorpora o `raw data` no HTML final.
pub fn file_base64(file: String, tipo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_data = fs::read(file.clone())
                        .expect("Arquivo de mídia não encontrado para converter em base64");

    let teste = file.clone();
    
    let file_extension = Path::new(&teste)
                        .extension()
                        .expect(&format!("Erro ao determinar o tipo de arquivo de: {}", &file))
                        .to_str()
                        .ok_or(&format!("Erro ao converter o caminho de {} para string.", &file))
                        .expect(&format!("Erro ao validar {} como caminho de arquivo", &file));
    Ok(format!("data:{}/{};base64,{}", tipo, file_extension, base64::encode(&file_data)))
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

/// Valida o arquivo de entrada como um stv válido. Atualmente, apenas checka se o arquivo 
/// é um .stv. 
//  TODO: adicionar capacidade de verificar sintaxe.
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
