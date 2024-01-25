//  Jefferson T. @StalinCCCP - 2024.

//! Confecciona um CSS padrão ou retorna de um arquivo externo indicado pela âncora `.css`.

/// Cria um estilo padrão de CSS para compilação quando no arquivo de entrada 
/// não for apresentado uma âncora do tipo `.style` apontando para um arquivo 
/// personalizado de CSS. 
///
/// O estilo padrão define comportamento de zoom, ajuste à página, estilo de 
/// fontes e outros.
pub fn generate_style(css_path: Option<String>) -> String {
    match css_path {
        Some(css_path) => format!("<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">", css_path),
        None => r#"
          <style>
          /* Licença no fim - Licence at the end */
          /* Jefferson T. 2024*/
          /* @stalincccp */
          
          body {
            text-align: justify;
            font-family: "Fira Sans", 'Lato', sans-serif;
            /* Importante zerar todas bordas */
            border: 0px;
            margin: 0px !important;
            padding: 0px !important;
            display: block;
            font-size: 24px;
          } 
          
          #popup {
            display: block;
            position: fixed;
            top: 1em;
            left: 1em;
            color: black;
            background-color: rgba(200, 225, 255, 0.90); 
            padding: 20px;
            border: 3px solid #000;
            border-radius: 5px;
            z-index: +100;
          }
          #popup td, tr, th {
            text-align: left;
            padding-right: 2em;
          }
          #popup h1{
            text-align: center;
          }
          #popup h2{
            text-align: center;
          }
          #popup p{
            text-align: center;
          }
          
          img {
            /*
            flex-grow: 1;
            */
            max-width: 100%;
            max-height: 90%;
            border-radius: 7px;
          }
          figcaption {
            color: black;
            font: italic smaller sans-serif;
            padding: 3px;
            text-align: center;
          }
          
          .slide {
            /*
            width: 100vw;
            slide-break-after: always;
            slide-break-before: always;
            break-after: always;
            */
            align-items: center;
            flex-direction: column;
            justify-content: center;
            display: flex;
            align-items: space-between;
            height: 95vh;
            overflow: hidden;
          }
          .slide p {
            padding: 1em;
          }
          .slide h1 {
            /* 
            */
          }
          .slide h2 {
            /* 
            */
          }
          .slide h1, h2, h3 {
            /*
            display: grid;
            align-items: center; 
            justify-content: center; 
            */
            text-align: center;
            margin: 0;
            padding-top: clamp(2%, 2vw, 80%);
            padding-right: 1vw;
            padding-left: 1vw;
            max-height: 100%; 
            overflow-y: auto; 
          }
          .slide video {
            width: 98vw;
            height: 98vh;
            object-fit: contain;
            padding-top: 1vh;
            padding-bottom: 1vh;
            padding-right: 1vw;
            padding-left: 1vw;
          }
          .slide th {
            background-color: gray;
            border: 1px solid;
            text-align: center;
            padding: 8px;
          }
          .slide td {
            border: 1px solid;
            padding: 8px;
            text-align: left;
          }
          .slide table {
            max-width: 90vw;
            margin: 0 auto;
            margin-top: 30vh;
            border-collapse: collapse;
          }
          
          .center {
            display: grid;
            justify-content: center;
            align-items: center;
          }
          
          .diviframe {
            width: 80vw;
            height: 80vh;
            padding-top: 10vh;
            padding-bottom: 10vh;
            padding-right: 10vw;
            padding-left: 10vw;
          }
          
          iframe {
            width: 100%;
            height: 100%;
          }
          
          .logo {
            position: absolute;
            right: 3px;
            top: 3px;
            width: 9%;
            height: auto;
            z-index: +100;
          }
          
          footer {
            position: fixed;
            font-size: 50%;
            background-color: rgba(200, 225, 255, 0.5); 
            left: 3rem;
            padding-right: 1em;
            padding-left: 1em;
            max-width: 50vw;
            bottom: 0;
            text-align: center;
            border-top: 2px solid #0060ab;
            color: black;
          }
          
          #marcador {
           position: absolute;
           width: 80px;
           height: 80px;
           border-radius: 50%;
           background-color: transparent;
           display: none; 
           border: 8px solid rgba(55, 0, 200, 1); 
           z-index: 9999; 
          }
          
          /* LISTAS */
          .listas {
            text-align: start;
            margin-left: auto;
            margin-right: auto;
            gap: 1em;
            display: flex;
            max-width: 90vw;
          }
          .listas ul, 
          .list ol {
            font-weight: normal;
            margin-left: auto;
            margin-right: auto;
            padding-left: 10vw;
          }
          
          .mermaidTooltip {
            display: none;
          }
          
          div.images {
            /*
            flex: 1;
            */
            display: inline-flex;
            align-content: center;
            justify-content: center;
            overflow: hidden;
            max-width: 100vw;
            max-height: 100vh;
            align-items: center;
          }
          
          </style>"#.to_string(),
    }
}

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
