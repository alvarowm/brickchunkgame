# BrickChunkGame

<p align="center">
<img src="https://github.com/alvarowm/brickchunkgame/blob/master/logo.png" width="512" height="512" />
</p>

Um emulador de Brick Game, baseado no microcontrolador HT1130/HT44300/443A0 feito inteiramente em Rust visando um código legível sem o uso extensivo de bitwise para as intruções.
<br>Todo o projeto é baseado neste incrível trabalho de engenharia reversa -> https://habr.com/ru/articles/773040/


## O que já foi implementado
- Fluxo de computação
- Estrutura do microcontrolador
- Todas as instruções do microcontrolador
- Rotina de interrupções em uma Thread separada
- Integração e uma interface mínima em Ratatui
- Decodificador de instruções
- Configurações são carregadas por meio de um properties, facilitando debug
- Mapeamento do espaço da ram reservada para saída do LCD
- Leitor da rom e inicialização, permitindo acesso a qualquer outro arquivo de rom

## O que precisa ser feito
- Integração do ratatui com o input que geram as interrupções
- Suporte a controles
- Interface amigável
- Ajustar timer
- Implementar as instruções de som

## Uso

```
brickchunkgame.exe -properties arquivo.properties -rom rom.bin
```
Caso não seja colocado nenhum parâmetro o emulador vai procurar no local do executável os arquivos:
- E23PlusMarkII96in1.bin
- application.properties

O arquivo de configuração suporta 4 opções até o momento:
- debug=?
- lcd=?
- debug_ram=?
- debug_vram=?

? true ou false

## Build
Você pode buildar sua aplicação por meio do cargo.
```
cargo build --release
```

## Contribuição

Fique à vontade para participar do projeto e ajudar a tornar ele jogável. ^^

</br>

<p align="center">
<img src="https://github.com/alvarowm/brickchunkgame/blob/master/rust_button_icon_151922.svg" width="512" height="188" />
</p>
