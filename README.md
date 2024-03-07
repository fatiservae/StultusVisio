<!--This file is part of StultusVisio.

    StultusVisio is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    StultusVisio is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with StultusVisio.  If not, see <https://www.gnu.org/licenses/>6.
    Jefferson T. @ 2023. Telegram: StalinCCCP 
-->
# StultusVisio 0.3.1
Slides estúpidos e fáceis, apresentáveis em qualquer _browser_.

## Exemplo
[Minha_apresentacao.html](./Exemplo/minha_apresentacao.html) 

## Racional
Todos atuais softwares de apresentação de slides são **péssimos**.

Um modo minimalista de construir apresentações, sem transições inúteis, modos contraproducentes como duas telas, WYSIWYG etc.

## Filosofia e uso
Um software simples que te obriga a montar slides coerentes.

Um slide deve conter apenas um vídeo, duas images ou texto. Evite inserir ambos.

Para fluxogramas, timelines, tabelas e mídias de dados organizados, use ![Mermaid](https://github.com/mermaid-js/mermaid), conforme exemplo abaixo.

Crie um arquivo como `minha_apresentacao.stv` e edite-o:

```
.title Título da apresentação
.foot Um rodapé.
# Estes são comentários porque começam com #. 
# As âncoras .script e .style apontam 
# para arquivos personalizados de 
# configuração. São opcionais.
.script ./Config/style.css
.style ./Config/script.js

--- 
.heading Os três hífens iniciam um slide.
.list
Primeiro item de uma lista.
Segundo item de uma lista.
Se quiser lista ordenada, use .ordlist
---
.image ./Images/exemplo.jpg
# um slide com imagem
---
.video ./Videos/exemplo.mp4
# slide com vídeo
---
.urlvideo "iframe do YouTube"
# esta âncora .urlvideo serve para integrar iframes
---
.heading Exemplo de fluxograma em mermaid
.mermaid
graph TD
A --- B
B-->C[fa:fa-ban Fim]
B-->D(fa:fa-spinner);
C-->A

---
.subheading Um slide com tabela
.table
Coluna 1 | Coluna 2 | Coluna 3
Item 1 | Item 2 | Item 3
Última 1 | Última 2 | Última 3
---
.heading Um exemplo de lista ordenada.
.ordlist
Primeiro item numerado.
Segundo.
Mais um.
Último.

```
No mesmo diretório do arquvio, execute o StultusVisio:
```
$ stultusvisio minha_apresentacao.stv
```
A apresentação em `HTML` será criada no mesmo diretório, com mesmo nome do arquivo.

Abra-a em qualquer navegador web. O arquivo é portável e único. Basta compartilhar o HTML em si.

Para personalizar estilos e comportamento, aponte para arquivos personalizados de `script.js` e `style.css`.

> [!NOTE]
> Erros de sintaxe no arquivo presentation.stv são renderizados como slides com uma mensagem de erro.

> [!IMPORTANT]
> A violação da filosofia do software produz comportamento imprevisto. 

## Controles

```
t   :   Abra/feche a ajuda.
j   :   Vá ao próximo slide.
k   :   Vá ao slide anterior.
p   :   Alterne para modo printável. 
gg  :   Vá ao primeiro slide.
G   :   Vá ao último slide.
m   :   Abra o marcador interativo.
x   :   Altere tamanho do marcador.

Deslize para passar slides em telas móveis com aparelho deitado.
```

## Salvar como PDF
Pressione `p` para alternar no modo printável. 
Em qualquer navegador, escolha modo `landscape` e em opções avançadas, desmarque `headers and footers` ou `cabeçalhos e rodapés`.
Imprima na opção `Salvar para PDF`.

## Tasks e devesenvolvimento
[Documentação](./target/doc/stultusvisio/index.html)

- [x] básico de uso geral.
- [x] marcador circular.
- [x] controles vim-like.
- [x] implementar opção STDIN file.
- [x] implementar tabelas.
- [ ] marcador do tipo desenho.
- [ ] zoom ad-hoc do slide atual.
- [ ] incluir arquivos de fontes.

## Licença
GNU GPL v.3
>This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with this program.  If not, see https://www.gnu.org/licenses/.
