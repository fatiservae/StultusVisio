/// Slide define um tipo interno que terá
/// sempre a última forma de String. Seu
/// resultado é a confecção do html final
/// do projeto.
#[derive(Debug, Default)] 
pub struct Slide {
    pub images: Option<Vec<String>>,
    //pub image1: Option<String>,
    //pub image2: Option<String>,
    //pub image3: Option<String>,
    //pub image4: Option<String>,
    pub video: Option<String>,
    pub caption: Option<String>,
}

/// Handle manipula o metadado da entrada 
/// atualmente manipulada, permitindo que
/// informações sobre o item inserido no
/// slide seja alterado, como legendas em
/// imagens, função de autoplay em videos
/// e outros.
pub enum Handle {
    // Um Handle::Slide so atrapalha o codebase
    // porque confunde com struct Slide. Trava
    // o uso de 'use lib::Handle::*'
    //Slide, 
    Image,
    Video,
    List,
    OrdList,
    Heading1,
    Heading2,
    Text,
    Caption,
}

pub fn trim_elements(element: &String) -> String {
    if let Some(index) = element.find(' ') {
        let cut_string = element[index + 1..].to_string();
        cut_string
    } else {
       element.to_string() // montar um erro aqui 
    }
}

pub fn new_slide() {
    println!("Novo slide aqui.")
}

pub fn insert_element(linha: String) -> String {
    return format!("TAG INSERIDA A DEPENDER DO TIPO")
}

pub fn print_slides(slide: Slide) {
    //fazer o mesmo pra video
    match slide.images{
        None => println!("nenhuma imagem"),
        Some(images) => match images.len() {
            1 => {
                    println!("<figure><img src=\"{}\" >", trim_elements(&images[0]));
                },
            2 => println!("duas imagens"),
            3 => println!("tres imagens"),
            4 => println!("quatro imagens"),
            _ => println!("mais de quatro imagens, NAO PODE!")
               // for image in slide.images {
               // println!("<div class=\"page\">{:?}", slide.image1)
               // }
        }
    }
}
