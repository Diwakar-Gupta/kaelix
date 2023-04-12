#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq)]
pub enum Task {
    SetCommand(String),
    AskInput(String),
    NewDoc,
    None,
    OpenDoc(String),
    NextTab,
    PrevTab,
    CloseCurrentTab,
}
