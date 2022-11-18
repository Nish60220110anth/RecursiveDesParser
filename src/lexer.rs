#![allow(non_snake_case)]


pub struct Lexer {
    pub content: Option<Vec<char>>,
    pub len: usize,
    pub index: usize,
}

pub struct LexState {
    pub index: usize,
}

impl From<&String> for Lexer {
    fn from(file_content: &String) -> Self {
        Lexer {
            content: Some(file_content.chars().collect()),
            len: file_content.len(),
            index: 0,
        }
    }
}

impl From<()> for Lexer {
    fn from(_: ()) -> Self {
        Lexer {
            content: None,
            len: 0,
            index: 0,
        }
    }
}

impl Lexer {
    pub fn getNextToken(&mut self) -> Option<char> {
        if self.isEmpty() {
            None
        } else {
            let pchar = self.content.as_ref().ok_or("None Value found").unwrap()[self.index];
            self.index += 1;
            Some(pchar)
        }
    }

    pub fn applyState(&mut self, state: LexState) -> () {
        self.index = state.index;
    }

    pub fn saveState(&self) -> LexState {
        LexState { index: self.index }
    }

    pub fn isEmpty(&self) -> bool {
        self.index == self.len
    }
}

pub mod RecDes {
    use super::Lexer;
    pub fn NStart(mut obj: Lexer) -> bool {
        NA(&mut obj)
    }

    pub fn NA(obj: &mut Lexer) -> bool {

        if obj.isEmpty() {
            return false;
        }

        let state = obj.saveState();
        if NB(obj) {
            if NC(obj) {
                return true;
            }
        }
        obj.applyState(state);
        return false;
    }

    pub fn NB(obj: &mut Lexer) -> bool {
        let state = obj.saveState();
        if ND(obj) {
            if NBD(obj) {
                return true;
            }
        }
        obj.applyState(state);
        return false;
    }

    pub fn NBD(obj: &mut Lexer) -> bool {
        if obj.isEmpty() {
            return true;
        }

        let state = obj.saveState();

        let pchar: char = obj.getNextToken().ok_or('c').expect("Oops");

        if pchar == '!' {
            if NBD(obj) {
                return true;
            }
        }
        obj.applyState(state);
        return true;
    }

    pub fn NC(obj: &mut Lexer) -> bool {
        if obj.isEmpty() {
            return true;
        }

        let state = obj.saveState();
        if let Some(pchar) = obj.getNextToken() {
            if pchar == '*' {
                if NA(obj) {
                    return true;
                }
            }
        }
        obj.applyState(state);
        return true;
    }

    pub fn ND(obj: &mut Lexer) -> bool {
        let state = obj.saveState();
        if !obj.isEmpty() {
            if let Some(pchar) = obj.getNextToken() {
                if pchar == 'n' {
                    return true;
                } else if pchar == '(' {
                    if NA(obj) {
                        if let Some(pchar) = obj.getNextToken() {
                            if pchar == ')' {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        obj.applyState(state);
        return false;
    }
}
