use uilang::uilang;

#[test]
fn simple() {
    uilang!(
        <Frame>
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
                    text: "This is button 2"
                </Button>
            </Frame>
        </Frame>
    );
}