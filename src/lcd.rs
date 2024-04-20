use std::fmt::Debug;
use bit::BitIndex;

#[derive(Debug)]
pub struct LcdScreen {
    rowsxcolumns: [[bool;10];20]
}
pub fn show(ram: [u8; 256]){
    let lcd = map_ram_to_lcd(ram);
    for i in 0..lcd.rowsxcolumns.len() {
        for j in 0..lcd.rowsxcolumns[i].len(){
            //let _ = print!("{}", if lcd.rowsxcolumns[i][j] {"■"} else {"□"});
            let _ = print!("{}", if lcd.rowsxcolumns[i][j] {"[]"} else {"X"});
        }
       println!();
    }
}

pub fn map_ram_to_lcd (ram: [u8; 256]) -> LcdScreen{
    let lcd_screen = LcdScreen{
        rowsxcolumns: [
            [ram[217].bit(3),ram[217].bit(2),ram[217].bit(1),ram[217].bit(0),ram[216].bit(3),ram[216].bit(2),ram[216].bit(1),ram[216].bit(0),ram[194].bit(3),ram[192].bit(3)],
            [ram[219].bit(3),ram[219].bit(2),ram[219].bit(1),ram[219].bit(0),ram[218].bit(3),ram[218].bit(2),ram[218].bit(1),ram[218].bit(0),ram[194].bit(0),ram[192].bit(0)],
            [ram[221].bit(3),ram[221].bit(2),ram[221].bit(1),ram[221].bit(0),ram[220].bit(3),ram[220].bit(2),ram[220].bit(1),ram[220].bit(0),ram[194].bit(2),ram[192].bit(2)],
            [ram[223].bit(3),ram[223].bit(2),ram[223].bit(1),ram[223].bit(0),ram[222].bit(3),ram[222].bit(2),ram[222].bit(1),ram[222].bit(0),ram[194].bit(1),ram[192].bit(1)],
            [ram[225].bit(3),ram[225].bit(2),ram[225].bit(1),ram[225].bit(0),ram[224].bit(3),ram[224].bit(2),ram[224].bit(1),ram[224].bit(0),ram[196].bit(0),ram[196].bit(1)],
            [ram[227].bit(3),ram[227].bit(2),ram[227].bit(1),ram[227].bit(0),ram[226].bit(3),ram[226].bit(2),ram[226].bit(1),ram[226].bit(0),ram[198].bit(0),ram[198].bit(1)],
            [ram[229].bit(3),ram[229].bit(2),ram[229].bit(1),ram[229].bit(0),ram[228].bit(3),ram[228].bit(2),ram[228].bit(1),ram[228].bit(0),ram[200].bit(0),ram[200].bit(1)],
            [ram[231].bit(3),ram[231].bit(2),ram[231].bit(1),ram[231].bit(0),ram[230].bit(3),ram[230].bit(2),ram[230].bit(1),ram[230].bit(0),ram[202].bit(0),ram[202].bit(1)],
            [ram[233].bit(3),ram[233].bit(2),ram[233].bit(1),ram[233].bit(0),ram[232].bit(3),ram[232].bit(2),ram[232].bit(1),ram[232].bit(0),ram[204].bit(0),ram[204].bit(1)],
            [ram[235].bit(3),ram[235].bit(2),ram[235].bit(1),ram[235].bit(0),ram[234].bit(3),ram[234].bit(2),ram[234].bit(1),ram[234].bit(0),ram[206].bit(0),ram[206].bit(1)],
            [ram[237].bit(3),ram[237].bit(2),ram[237].bit(1),ram[237].bit(0),ram[236].bit(3),ram[236].bit(2),ram[236].bit(1),ram[236].bit(0),ram[208].bit(0),ram[208].bit(1)],
            [ram[239].bit(3),ram[239].bit(2),ram[239].bit(1),ram[239].bit(0),ram[238].bit(3),ram[238].bit(2),ram[238].bit(1),ram[238].bit(0),ram[210].bit(0),ram[210].bit(1)],
            [ram[241].bit(3),ram[241].bit(2),ram[241].bit(1),ram[241].bit(0),ram[240].bit(3),ram[240].bit(2),ram[240].bit(1),ram[240].bit(0),ram[214].bit(2),ram[212].bit(2)],
            [ram[243].bit(3),ram[243].bit(2),ram[243].bit(1),ram[243].bit(0),ram[242].bit(3),ram[242].bit(2),ram[242].bit(1),ram[242].bit(0),ram[214].bit(0),ram[212].bit(0)],
            [ram[245].bit(3),ram[245].bit(2),ram[245].bit(1),ram[245].bit(0),ram[244].bit(3),ram[244].bit(2),ram[244].bit(1),ram[244].bit(0),ram[214].bit(1),ram[212].bit(1)],
            [ram[247].bit(3),ram[247].bit(2),ram[247].bit(1),ram[247].bit(0),ram[246].bit(3),ram[246].bit(2),ram[246].bit(1),ram[246].bit(0),ram[214].bit(3),ram[212].bit(3)],
            [ram[249].bit(3),ram[249].bit(2),ram[249].bit(1),ram[249].bit(0),ram[248].bit(3),ram[248].bit(2),ram[248].bit(1),ram[248].bit(0),ram[215].bit(0),ram[213].bit(0)],
            [ram[251].bit(3),ram[251].bit(2),ram[251].bit(1),ram[251].bit(0),ram[250].bit(3),ram[250].bit(2),ram[250].bit(1),ram[250].bit(0),ram[215].bit(1),ram[213].bit(1)],
            [ram[253].bit(3),ram[253].bit(2),ram[253].bit(1),ram[253].bit(0),ram[252].bit(3),ram[252].bit(2),ram[252].bit(1),ram[252].bit(0),ram[215].bit(3),ram[213].bit(3)],
            [ram[255].bit(3),ram[255].bit(2),ram[255].bit(1),ram[255].bit(0),ram[254].bit(3),ram[254].bit(2),ram[254].bit(1),ram[254].bit(0),ram[215].bit(2),ram[213].bit(2)]]
    };
    return lcd_screen;
}