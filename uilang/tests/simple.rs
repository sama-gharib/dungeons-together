use uilang::uilang;
use desi_ui::*;
use macroquad::prelude::*;

#[test]
fn simple() {
    let mut a = uilang!{
        <Frame>
            center: "(0.5, 0.5)"
            scale: "(1.0, 1.0)"
            primary: "RED"
            secondary: "BLUE"
            <Label>
                text: "This is a title"
            </Label>
            <Frame>
                <Button>
                    <Label>
                        text: "This is button 1"
                    </Label>
                </Button>
                <Button>
                    <Label>
                        text: "This is button 2"
                    </Label>
                </Button>
            </Frame>
        </Frame>
    };    
}