use std::fs;
use base64;

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
}

pub fn trim_element(input: &String) -> String {
    if let Some(index) = input.find(' ') {
        let cut_string = input[index + 1..].to_string();
        cut_string
    } else {
        input.to_string() //isso deve ser um erro!
    }
}

pub fn close_last_handle(handle: &Option<Handle>) -> &str {
    match handle {
        None => "",
        Some(Handle::Image) => "</figure>",
        Some(Handle::Caption) => "</figure>",
        Some(Handle::List) => "</ul>",
        Some(Handle::OrdList) => "</ol>",
        Some(Handle::Text) => "</p>",
        Some(Handle::Heading) => "</h1>",
        Some(Handle::SubHeading) => "</h2>",
        Some(Handle::Video) => "</video>",
        Some(Handle::URLVideo) => "</iframe>",
    }
}

pub fn file_base64(file: String) -> Result<String, Box<dyn std::error::Error>> {
    let file_data = fs::read(file)?;
    Ok(base64::encode(&file_data))
}

pub fn generate_script(script_path: Option<String>) -> String {
    match script_path {
        Some(script_path) => format!("<script src=\"{}\"></script>", script_path),
        None => "<script>    
         var slides = document.querySelectorAll('.slide');
         var currentslideIndex = 0;
         var n = 0;
         
         const popup = document.getElementById(\"popup\");
         const popupText = document.getElementById(\"conteudo-popup\");

         function popUpShow(){
             popupText.innerHTML = `<h1>Ajuda</h1>
                          <h2>Slide ${currentslideIndex+1} de ${slides.length}</h2>
                          <table>
                          <tr>
                            <th>Comando</th><th>Tecla</th>
                          </tr>
                          <tr>
                            <td>Ajuda</td><td>t</td>
                          </tr>
                          <tr>
                            <td>Próximo</td><td>j</td>
                          </tr>
                          <tr>
                            <td>Anterior</td><td>k</td>
                          </tr>
                          <tr>
                            <td>Topo</td><td>gg</td>
                          <tr>
                          </tr>
                            <td>Final</td><td>G</td>
                          </tr>
                            <td>Modo impressão</td><td>p</td>
                          </tr>

                          </table>
                        `;
         }
         popUpShow();

         function Printar() {
           for (var j = 0; j < slides.length; j++) {
               slides[j].style.display= 'block';
           }
         };

         document.addEventListener('keydown', function(event) {
           if (event.key === 'ArrowRight' || event.key === 'j') {
             if (currentslideIndex < slides.length - 1){
               var contador = parseInt(n);
               if (contador === 0) {contador = contador + 1}
               else if (contador > slides.length) {contador = 0};
               currentslideIndex = currentslideIndex + contador;
               n = 0;
             }
           }
           else if (event.key === 'ArrowLeft' || event.key === 'k'){
             if (currentslideIndex > 0) {
              var contador = parseInt(n);
              if (contador === 0) {contador = contador + 1};
              currentslideIndex = currentslideIndex - contador;
              if (currentslideIndex < 0 ){currentslideIndex = 0};
              n = 0;
             }
           }
           else if (event.key === 'p') {
             Printar();
             return;
           }
           else if (event.key === 't') {
             if (popup.style.display === 'none') {popup.style.display = \"block\"}
             else {popup.style.display = \"none\"};
             popUpShow()
           }
           else if (typeof event.key === 'string' && event.key >= '0' && event.key <= '9'){
             if (n === 0) {n = `${event.key}`}else{n = `${n}${event.key}`}
           }

           else if (event.key === 'g'){
             document.addEventListener('keydown', function(event) {
                if (event.key === 'g') {currentslideIndex = 0};
             })
           }
           
           else if (event.key === 'G') { currentslideIndex = slides.length - 1}

           else if (event.key === 'm'){toggleMovement()}

           else if (isMoving && event.key === 'ArrowUp' || event.key === 'ArrowLeft' || event.key === 'ArrowDown' || event.key === 'ArrowRight') {moveCircle(event.key)}

           else if (event.key === 'x') {resizeMarcador()}

           else {return};

           for (var i = 0; i < slides.length; i++) {
             if (i === currentslideIndex) {
               slides[i].style.display = 'block';
             } else {
               slides[i].style.display = 'none';
             }
           }
         });

         const circle = document.getElementById('marcador');
         var tamanhoMarcador = 1;
         let circleTop = 0;
         let circleLeft = 0;
         let isMoving = false;

         function updateCirclePosition() {
           circle.style.top = circleTop + 'px';
           circle.style.left = circleLeft + 'px';
         }

         function resizeMarcador() {
           if (tamanhoMarcador > 250) {tamanhoMarcador = 5} else {tamanhoMarcador = tamanhoMarcador + 5};
           circle.style.width = tamanhoMarcador + 'px';
           circle.style.height = tamanhoMarcador + 'px';
         }
         
         document.addEventListener('mousemove', function(event) {
           const x = event.clientX;
           const y = event.clientY;
           marcador.style.transform = `translate(${x}px, ${y}px)`;
         });

         function moveCircle(direction) {
           const step = 10; 
           switch (direction) {
             case 'ArrowDown':
               circleTop += step;
               break;
             case 'ArrowUp':
               circleTop -= step;
               break;
             case 'ArrowLeft':
               circleLeft -= step;
               break;
             case 'ArrowRight':
               circleLeft += step;
               break;
           }

           updateCirclePosition();
         }

         function toggleMovement() {
           isMoving = !isMoving;
           if (isMoving) {
             circle.style.display = 'block';
           } else {
             circle.style.display = 'none';
           }
         }
         </script>
".to_string(),
    }
}

pub fn generate_logo(logo_path: Option<String>) -> String {
    match logo_path {
        Some(logo_path) => format!("<img src=\"{}\" class=\"logo\">", logo_path),
        None => "".to_string(),
    }
}

pub fn generate_style(css_path: Option<String>) -> String {
    match css_path {
        Some(css_path) => format!("<link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">", css_path),
        None => "<style>@font-face {
  font-family: 'Arial';
  /*src: url('./SF-Pro-Display-Medium.otf') format('opentype');*/
}

body {
  display:grid;
  margin: 0 !important;
  padding: 0 !important;
  max-width: 90vw;
  border: none;
  font-family: 'SFPro', Arial, sans-serif;
  font-size: clamp(12px, 1em, 5vw);
  text-align: justify;
}

#popup {
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
  font-size: 2em;
}

#popup h2{
  text-align: center;
  font-size: 1.5em;
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

h1, h2, h3 {
  font-size: calc(2vw + 2vh + 1vmin); /* cálculo responsivo */
  max-height: 100%; /* Define a altura máxima igual à altura do elemento pai */
  overflow-y: auto; /* Adiciona uma barra de rolagem vertical quando necessário */
  display: grid;
  align-items: center; /* Centraliza verticalmente */
  justify-content: center; /* Centraliza horizontalmente */
}

ul, ol {
  padding-left: 5vw;
  padding-right: 5vw;
  font-size: calc(1vw + 1vh + 1vmin); 
  max-height: 100%; 
  overflow-y: auto; 
  display: grid;
  align-items: center; 
  justify-content: center; 
}

.slide figure {
  object-fit: contain;
  display: flex;
  flex-direction: column;
  justify-content: center;
  height: 95vh;
  max-width: 100%;
  max-height: 100%;
  border-radius: 10px 10px 10px 10px;
}

.slide figure figcaption {
/*  text-shadow: 2px 2px 4px rgba(3, 3, 3, 0.5);*/
  position: relative;
  bottom: 0em;
  font-size: 1em;
  text-align: right;
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
  padding-top: 1vh;;
  padding-bottom: 1vh;;
  padding-right: 1vw;;
  padding-left: 1vw;;
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
  background-color: rgba(200, 225, 255, 0.5); 
  left: 3em;
  right: 65vw;
  border-top: 1px solid green;
  bottom: 0;
  text-align: center;
  position: fixed;
  font-size: 0.8em;
  color: black;
}

/* Mostra apenas a primeira página
.slide:nth-of-type(1){
  display: block;
  background-image: url('capa.jpg');
  background-size: cover;
  background-position: center; 
}
*/
.titulo {
  padding-top: 10vh;
  color: black; 
  text-shadow: 2px 2px 4px rgba(250, 250, 250, 3.5);
}

/*
.capa {
  height: 100%;
  max-height: 100%;
  width: 100%;
  background-image: url('capa.jpg');
  background-size: cover;
  background-position: center; 
}*/

/*#marcador {
  position: absolute;
  width: 50px;
  height: 50px;
  border: 2px solid red;
  border-radius: 50%;
  display: none; /* Oculta o círculo inicialmente 
  z-index: +100;
}

/*seta*/
/*
#marcador {
  position: absolute;
  width: 0;
  height: 0;
  border-top: 10px solid transparent;
  border-bottom: 10px solid transparent;
  border-right: 20px solid red;
  transform-origin: left center;
  transition: transform 0.4s ease-out;
  z-index: +100;
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

/*.capa img{
  top:0;
  left:0;
  display: absolute;
  z-index: -1;
  height: 100vh;
  width: auto;
  max-width: 100%;
  max-height: 100%;
}
.capa ul{
  top:0;
  left:0;
  display: absolute;
  color: white; 
  text-shadow: 2px 2px 4px rgba(1, 1, 1.5, 3.5);
  object-fit: contain;
  z-index: +1;
  font-size: 1em;
}
*/
</style>".to_string(),
    }
}
