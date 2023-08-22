    var pages = document.querySelectorAll('.page');
    var currentPageIndex = 0;
    var n = 0;
    
    const popup = document.getElementById("popup");
    const popupText = document.getElementById("conteudo-popup");

    function popUpShow(){
        popupText.innerHTML = `<h1>Ajuda</h1>
                     <h2>Slide ${currentPageIndex+1} de ${pages.length}</h2>
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
      for (var j = 0; j < pages.length; j++) {
          pages[j].style.display= 'block';
      }
    };

    document.addEventListener('keydown', function(event) {
      if (event.key === 'ArrowRight' || event.key === 'j') {
        if (currentPageIndex < pages.length - 1){
          var contador = parseInt(n);
          if (contador === 0) {contador = contador + 1}
          else if (contador > pages.length) {contador = 0};
          currentPageIndex = currentPageIndex + contador;
          n = 0;
        }
      }
      else if (event.key === 'ArrowLeft' || event.key === 'k'){
        if (currentPageIndex > 0) {
         var contador = parseInt(n);
         if (contador === 0) {contador = contador + 1};
         currentPageIndex = currentPageIndex - contador;
         if (currentPageIndex < 0 ){currentPageIndex = 0};
         n = 0;
        }
      }
      else if (event.key === 'p') {
        Printar();
        return;
      }
      else if (event.key === 't') {
        if (popup.style.display === 'none') {popup.style.display = "block"}
        else {popup.style.display = "none"};
        popUpShow()
      }
      else if (typeof event.key === 'string' && event.key >= '0' && event.key <= '9'){
        if (n === 0) {n = `${event.key}`}else{n = `${n}${event.key}`}
      }

      else if (event.key === 'g'){
        document.addEventListener('keydown', function(event) {
           if (event.key === 'g') {currentPageIndex = 0};
        })
      }
      
      else if (event.key === 'G') { currentPageIndex = pages.length - 1}

      else if (event.key === 'm'){toggleMovement()}

      else if (isMoving && event.key === 'ArrowUp' || event.key === 'ArrowLeft' || event.key === 'ArrowDown' || event.key === 'ArrowRight') {moveCircle(event.key)}

      else if (event.key === 'x') {resizeMarcador()}

      else {return};

      for (var i = 0; i < pages.length; i++) {
        if (i === currentPageIndex) {
          pages[i].style.display = 'block';
        } else {
          pages[i].style.display = 'none';
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
