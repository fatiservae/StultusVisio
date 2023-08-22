# Stultus Visio
#Slides estúpidos e fáceis, apresentáveis em qualquer _browser_.

## Racional
Todos atuais softwares de apresentação de slides são péssimos.

Um modo minimalista de construir apresentações, sem transições inúteis, modos contraproducentes como duas telas, WYSIWYG e compiladores complexos.

## Filosofia e uso
Um software simples que te obriga a montar slides coerentes.

Um slide deve conter um(a) imagem, vídeo ou texto. Evite inserir ambos.

No diretório, crie o arquivo `presentation.stv` e edite-o:

```
.title Título da apresentação
.foot Um rodapé.
# Estes são comentários. 
# As tags .script e .style apontam 
# para arquivos personalizados de 
# configuração. São opcionais.
.script ./Config/style.css
.style ./Config/script.js

--- 
.heading Os três hífens iniciam um slide.
.list
Primeiro item de uma lista.
Segundo item de uma lista.

---
.image ./Images/exemplo.jpg

---
.video ./Imagens/exemplo.mp4

---
.heading Um exemplo de lista ordenada.
.ordlist
Primeiro item numerado.
Segundo.
Mais um.
Último.

```
Compile o StultusVisio:
```
$ cargo build --release 
```

Execute stultus_visio no diretório, por exemplo:
```
$ ./target/stultus_visio > Presentation.html
```

Abra `Presentation.html` em qualquer navegador web.

> [!NOTE]
> Erros de sintaxe no arquivo presentation.stv são renderizados como slides com mensagem de erro.

> [!IMPORTANT]
> A violação da filosofia do software produz comportamento imprevisto.

## Controles

```
t   :   Abre/fecha ajuda.
j   :   Próximo slide.
k   :   Antecede slide.
p   :   Alterna para modo printável. 
gg  :   Retorna ao primeiro slide.
G   :   Vá ao último slide.
m   :   Abre marcador interativo.
x   :   Altera tamanho do marcador.

```

## Salvar como PDF
Pressione `p` para alternar no modo printável. Em qualquer navegador, escolha modo `landscape` e em opções avançadas, desmarque `headers and footers` ou `cabeçalhos e rodapés`.

## Licença
GNU GPL v.3
>This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with this program.  If not, see https://www.gnu.org/licenses/.
