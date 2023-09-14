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
# StultusVisio 0.2.3

Slides estúpidos e fáceis, apresentáveis em qualquer _browser_.

## Racional
Todos atuais softwares de apresentação de slides são **péssimos**.

Um modo minimalista de construir apresentações, sem transições inúteis, modos contraproducentes como duas telas, WYSIWYG e compiladores complexos.

## Filosofia e uso
Um software simples que te obriga a montar slides coerentes.

Um slide deve conter apenas um(a) imagem, vídeo ou texto. Evite inserir ambos.

Para fluxogramas, timelines, tabelas e mídias de dados organizados, use ![Mermaid](https://github.com/mermaid-js/mermaid), conforme exemplo abaixo.

No diretório, crie um arquivo `minha_apresentacao.stv` e edite-o:

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
.video ./Videos/exemplo.mp4

---
.heading Exemplo de fluxograma em mermaid
.mermaid
graph TD
A --- B
B-->C[fa:fa-ban Fim]
B-->D(fa:fa-spinner);
C-->A

---
.heading Um exemplo de lista ordenada.
.ordlist
Primeiro item numerado.
Segundo.
Mais um.
Último.

```
## Compilação, instalação e uso

Compile o StultusVisio:
```
$ cargo build --release 
```

Mova o binário para $PATH ou execute stultus_visio no mesmo diretório do arquvio `minha_apresentacao.stv`, por exemplo:
```
$ ./target/stultus_visio minha_apresentacao.stv
```

A apresentação [minha_apresentacao.html](./minha_apresentacao.html) será criada no mesmo diretório.

Abra-a em qualquer navegador web.

O arquivo é portável e único. Basta compartilhar o HTML em si.

Para personalizar estilos e comportamento, aponte para arquivos personalizados de `script.js` e `style.css`.

> [!NOTE]
> Erros de sintaxe no arquivo presentation.stv são renderizados como slides com uma mensagem de erro.

> [!IMPORTANT]
> A violação da filosofia do software produz comportamento imprevisto. Não seja estúpido, apresentadores de slides não são editores de imagens. Edite-as no seu software favorito e importe-as.

## Controles

```
t   :   Abra/feche a ajuda.
j   :   Vá ao próximo slide.
k   :   Anteceda ao slide anterior.
p   :   Alterne para modo printável. 
gg  :   Vá ao primeiro slide.
G   :   Vá ao último slide.
m   :   Abra o marcador interativo.
x   :   Altere tamanho do marcador.

```

## Salvar como PDF
Pressione `p` para alternar no modo printável. 
Em qualquer navegador, escolha modo `landscape` e em opções avançadas, desmarque `headers and footers` ou `cabeçalhos e rodapés`.
Imprima na opção `Salvar para PDF`.

## Tasks e devesenvolvimento
[Documentação](./target/doc/stultus_visio/index.html)

- [x] básico de uso geral.
- [x] marcador circular.
- [x] controles vim-like.
- [x] implementar opção STDIN file.
- [ ] marcador do tipo desenho.
- [ ] zoom ad-hoc do slide atual.
- [ ] carrossel de imagens em único slide.
- [ ] incluir arquivos diretos de fontes.

## Licença
GNU GPL v.3
>This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with this program.  If not, see https://www.gnu.org/licenses/.
