use ratatui::{prelude::*, widgets::*};

pub struct LCD_screen {
    rowsxcolumns: [[bool;10];20]

}
pub fn show(ram: [u8; 256]){


}

pub fn map_ram_to_lcd (ram: [u8; 256]) -> LCD_screen{
    let lcd_screen = LCD_screen{
        rowsxcolumns: [
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false],
            [false,false,false,false,false,false,false,false,false,false]]
    };
    return lcd_screen;
}