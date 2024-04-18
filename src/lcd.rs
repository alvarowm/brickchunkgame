use ratatui::{prelude::*, widgets::*};
pub fn show(ram: [u8; 256]){

    Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::THICK)
        .padding(Padding::horizontal(1))
        .border_style(Style::new().blue().on_white().bold().italic());

    //linhas
    for i in 0..30 {
        //colunas
        for j in 0..20{

        }
    }


}