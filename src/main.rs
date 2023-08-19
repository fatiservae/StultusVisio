use std::fs::File;
use std::io::{BufRead, BufReader};
mod lib;
use lib::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.stv")?;
    let reader = BufReader::new(file);
 
    let mut slides: Vec<Slide> = vec![];
    fn last_number(vector: &Vec<Slide>) -> usize {
        if vector.len() == 0 {0} else { vector.len() - 1}
    }
    let mut last = 0;
    let mut handle: Option<Handle> = None;

    let mut images: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("---") {
            let mut slide = Slide {
                images: None,
                video: None,
                caption: None,
            };
            slides.push(slide);
            last = last_number(&slides);
            handle = None;
            images = vec![];

        } else if line.is_empty() | line.starts_with("#") {
            continue

        } else if line.starts_with(".image"){
            images.push(line);
            slides[last].images = Some(images.clone());
            handle = Some(Handle::Image);
            //match handle {
            //    Some(Handle::image) => { 
            //        continue
            //            // tá errado, nao era pra ser
            //    },
            //    _ => {
            //        handle = Some(Handle::image)
            //    },
            //}

        } else if line.starts_with(".caption"){
            match handle {
                Some(Handle::Image) | Some(Handle::Caption) => { 
                    slides[last].caption = Some(insert_element(line))
                },
                _ => {
                    println!("ERRO: aqui nao pode legenda.")
                },
            }

            // TODO: Opções de vídeo como autoplay.
        } else if line.starts_with(".video"){
            slides[last].video = Some(insert_element(line));
            handle = Some(Handle::Video);
        }
    }
    for slide in slides {
        print_slides(slide);
    }
    Ok(())
}
