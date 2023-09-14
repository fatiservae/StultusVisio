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

/// Cria um estilo padrão de CSS para compilação quando no arquivo de entrada 
/// não for apresentado uma âncora do tipo `.style` apontando para um arquivo 
/// personalizado de CSS. 
///
/// O estilo padrão define comportamento de zoom, ajuste à página, estilo de 
/// fontes e outros.
pub fn generate_style(css_path: Option<String>) -> String {
    match css_path {
        Some(css_path) => format!("<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">", css_path),
        None => "<style>
body {
  display:grid;
  margin: 0 !important;
  padding: 0 !important;
  max-width: 90vw;
  border: none;
  font-family: Heveltica, 'SFPro', sans-serif;
  font-size: clamp(15px, 30px, 50px);
  /*font-size: 16px;*/
  text-align: justify;
}

#popup {
  font-size: clamp(10%, 40%, 60%);
  display: block;
  position: fixed;
  top: 1em;
  left: 1em;
  color: black;
  background-color: rgba(200, 225, 255, 0.85); 
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

.slide {
  position: relative;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  max-height: 100vh; !important
  slide-break-after: always;
  slide-break-before: always;
  break-after: always;
}

.slide p {
  padding: 1em;
}

.slide h1, h2, h3 {
  padding-top: clamp(2%, 2vw, 80%);
  padding-right: 1vw;
  padding-left: 1vw;
  /*font-size: clamp(15px, 2em, 80%);*/
  max-height: 100%; /* Define a altura máxima igual à altura do elemento pai */
  overflow-y: auto; /* Adiciona uma barra de rolagem vertical quando necessário */
  display: grid;
  align-items: center; /* Centraliza verticalmente */
  justify-content: center; /* Centraliza horizontalmente */
}

.slide ol, ul {
  font-size: clamp(0.3em, 2.5em, 90%);
  padding-top: clamp(1px, 1em, 10%);
  padding-left: clamp(1%, 20%, 80%);
  padding-right: clamp(1px, 5%, 50%);
  display: inline-grid;
}

.slide figure {
  object-fit: contain;
  display: flex;
  flex-direction: column;
  justify-content: center;
  height: clamp(40vh, 80%, 95vh);
  max-width: 100%;
  max-height: 100%;
  border-radius: 10px 10px 10px 10px;
}

.slide figure figcaption {
/*  text-shadow: 2px 2px 4px rgba(3, 3, 3, 0.5);*/
  position: relative;
  bottom: 0em;
  text-align: right;
  padding-right: 20%;
  font-size: clamp(15%, 50%, 70%);
}

.slide figure img {
  object-fit: contain;
  max-width: 100%;
  max-height: 100%;
  object-position: 50% 50%;
  border-radius: 10px 10px 10px 10px;
}

img {
  border-radius: 10px 10px 10px 10px;
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

.center {
  display: grid;
  justify-content: center;
  align-items: center;
  /*height: 100vh;  Adjust to your needs */
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
  font-size: 35%;
  background-color: rgba(200, 225, 255, 0.5); 
  left: 3em;
  right: 65vw;
  bottom: 0;
  border-top: 2px solid green;
  text-align: center;
  position: fixed;
  color: black;
}

/*
.titulo {
  padding-top: 10vh;
  color: black; 
  text-shadow: 2px 2px 4px rgba(250, 250, 250, 3.5);
}
*/

#marcador {
 position: absolute;
 width: 80px;
 height: 80px;
 border-radius: 50%;
 background-color: transparent;
 display: none; /* Oculta o círculo inicialmente*/
 border: 8px solid rgba(55, 0, 200, 1); /* Cor da cobertura translúcida */
 z-index: 9999; /* Coloca o círculo acima de outros elementos */
}

</style>".to_string(),
    }
}
