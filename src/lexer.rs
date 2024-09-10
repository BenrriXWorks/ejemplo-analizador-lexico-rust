
use std::collections::HashMap;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum Error {
    EOF,
    Unmatch,
}

pub struct Lexer<'a> {
    input: &'a str,
    cmap: Vec<usize>,

    cmap2: HashMap<usize, usize>,

    start: CharIndices<'a>,
    current: CharIndices<'a>,
    max_len: usize,


    zz_state: usize,
    zz_lexical_state: usize,

    // byte
    zz_marked_pos: usize,
    zz_current_pos: usize,
    zz_start_read: usize,

    // char
    zz_start_read_char: usize,
    zz_marked_char: usize,

    zz_at_eof: bool,

}

impl<'a> Lexer<'a> {
    pub const ZZ_ROW: [usize; 29] = [0, 19, 38, 57, 76, 95, 114, 133, 19, 19, 19, 19, 152, 171, 190, 209, 228, 19, 247, 266, 285, 304, 323, 342, 19, 19, 361, 19, 19];
    pub const ZZ_TRANS: [i32; 380] = [1, 2, 1, 3, 1, 1, 1, 4, 5, 1, 1, 6, 7, 8, 9, 10, 11, 12, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 13, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 14, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 16, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 17, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 12, -1, -1, -1, -1, 18, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 19, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 21, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 22, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 23, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 24, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 25, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 26, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 27, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 28, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    pub const ZZ_ATTR: [i32; 29] = [0, 9, 1, 1, 1, 1, 1, 1, 9, 9, 9, 9, 1, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 9, 9, 0, 9, 9];
    pub const ZZ_ACTION: [i32; 29] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 14, 15, 0, 16, 17];
    pub const ZZ_LEXSTATE: [i32; 2] = [0, 0];
    pub const YYINITIAL: usize = 0;


    pub const YYEOF: i32 = -1;

    pub fn new(input: &'a str) -> Lexer<'a> {
        let max_len = input.chars().clone().count();
        let mut cmap: Vec<usize> = Vec::with_capacity(256);
        cmap.resize(256, 0);
        let mut cmap2: HashMap<usize, usize> = HashMap::new();
        cmap[10] = 18;
        cmap[11] = 18;
        cmap[12] = 18;
        cmap[13] = 18;
        cmap[32] = 17;
        cmap[40] = 14;
        cmap[41] = 15;
        cmap[44] = 16;
        cmap[48] = 12;
        cmap[49] = 12;
        cmap[50] = 12;
        cmap[51] = 12;
        cmap[52] = 12;
        cmap[53] = 12;
        cmap[54] = 12;
        cmap[55] = 12;
        cmap[56] = 12;
        cmap[57] = 12;
        cmap[61] = 13;
        cmap[65] = 11;
        cmap[66] = 11;
        cmap[67] = 11;
        cmap[68] = 11;
        cmap[69] = 11;
        cmap[70] = 11;
        cmap[71] = 11;
        cmap[72] = 11;
        cmap[73] = 11;
        cmap[74] = 11;
        cmap[75] = 11;
        cmap[76] = 11;
        cmap[77] = 11;
        cmap[78] = 11;
        cmap[79] = 11;
        cmap[80] = 11;
        cmap[81] = 11;
        cmap[82] = 11;
        cmap[83] = 11;
        cmap[84] = 11;
        cmap[85] = 11;
        cmap[86] = 11;
        cmap[87] = 11;
        cmap[88] = 11;
        cmap[89] = 11;
        cmap[90] = 11;
        cmap[97] = 1;
        cmap[98] = 6;
        cmap[100] = 10;
        cmap[101] = 7;
        cmap[102] = 3;
        cmap[103] = 4;
        cmap[110] = 2;
        cmap[114] = 5;
        cmap[116] = 9;
        cmap[119] = 8;
        cmap[133] = 18;
        cmap2.insert(8232, 18);
        cmap2.insert(8233, 18);


        Lexer {
            input,
            cmap,

            cmap2,

            start: input.char_indices(),
            current: input.char_indices(),

            max_len,
            zz_state: 0,
            zz_lexical_state: Lexer::YYINITIAL,
            zz_marked_pos: 0,
            zz_current_pos: 0,
            zz_start_read: 0,
            zz_start_read_char: 0,
            zz_marked_char: 0,

            zz_at_eof: false,

        }
    }


    #[allow(dead_code)]
    pub fn is_eof(&self) -> bool {
        self.zz_at_eof
    }

    #[allow(dead_code)]
    pub fn yybegin(&mut self, new_state: usize) {
        self.zz_lexical_state = new_state;
    }

    #[allow(dead_code)]
    pub fn yystate(&self) -> usize {
        self.zz_lexical_state
    }

    #[allow(dead_code)]
    pub fn yylength(&self) -> usize {
        self.zz_marked_char - self.zz_start_read_char
    }

    #[allow(dead_code)]
    pub fn yycharat(&self, pos: usize) -> Option<char> {
        let mut ch: Option<char> = None;
        let mut start = self.start.clone();
        for _ in 0..(pos + 1) {
            if let Some(c) = start.next() {
                ch = Some(c.1);
            } else {
                return None;
            }
        }
        ch
    }

    #[allow(dead_code)]
    pub fn yytext(&self) -> String {
        self.input[self.yybytepos()].to_string()
    }

    #[allow(dead_code)]
    pub fn yytextpos(&self) -> std::ops::Range<usize> {
        std::ops::Range {
            start: self.zz_start_read_char,
            end: self.zz_marked_char,
        }
    }

    #[allow(dead_code)]
    pub fn yybytepos(&self) -> std::ops::Range<usize> {
        std::ops::Range {
            start: self.zz_start_read,
            end: self.zz_marked_pos,
        }
    }

    #[allow(dead_code)]
    pub fn yylex(&mut self) -> Result<i32, Error> {
        let mut zz_input: i32 = -1;

        // cached
        loop {
            // char unit
            let mut zz_marked_char_l = self.zz_marked_char;
            let mut zz_current_char_pos_l = self.zz_marked_char;
            self.zz_start_read_char = self.zz_marked_char;

            // byte unit
            let mut zz_marked_byte_pos_l = self.zz_marked_pos;
            let mut zz_current_byte_pos_l = self.zz_marked_pos;

            let mut zz_action = -1;
            let mut current = self.current.clone();
            

            self.zz_start_read = self.zz_marked_pos;
            self.zz_current_pos = self.zz_marked_pos;
            self.zz_start_read_char = self.zz_marked_char;
            self.start = self.current.clone();

            self.zz_state = Lexer::ZZ_LEXSTATE[self.zz_lexical_state] as usize;

            // set up zz_action for empty match case:
            let zz_attributes = Lexer::ZZ_ATTR[self.zz_state];
            if (zz_attributes & 1) == 1 {
                zz_action = self.zz_state as i32;
            }

            'zz_for_action: loop {
                if zz_current_char_pos_l < self.max_len {
                    
                if let Some(next) = current.next() {
                    zz_current_byte_pos_l += next.1.len_utf8();
                    zz_input = next.1 as i32;
                }
                    zz_current_char_pos_l += 1;
                } else if self.zz_at_eof {
                    zz_input = Lexer::YYEOF;
                    break 'zz_for_action;
                } else {
                    self.zz_current_pos = zz_current_byte_pos_l;

                    if self.max_len <= zz_current_char_pos_l {
                        zz_input = Lexer::YYEOF;
                        break 'zz_for_action;
                    } else {
                        
                if let Some(next) = current.next() {
                    zz_current_byte_pos_l += next.1.len_utf8();
                    zz_input = next.1 as i32;
                }
                        zz_current_char_pos_l += 1;
                    }
                }

                let cidx = if zz_input <= 0xFF {
                    self.cmap[zz_input as usize]
                } else {

                    *self.cmap2.get(&(zz_input as usize)).unwrap_or(&0usize)

                };
                let idx = Lexer::ZZ_ROW[self.zz_state] + cidx;
                let zz_next = Lexer::ZZ_TRANS[idx];
                if zz_next == -1 {
                    break 'zz_for_action;
                }
                self.zz_state = zz_next as usize;

                let zz_attributes = Lexer::ZZ_ATTR[self.zz_state];
                if (zz_attributes & 1) == 1 {
                    zz_action = self.zz_state as i32;
                    zz_marked_char_l = zz_current_char_pos_l;
                    zz_marked_byte_pos_l = zz_current_byte_pos_l;
                    self.current = current.clone();

                    if (zz_attributes & 8) == 8 {
                        break 'zz_for_action;
                    }
                }
            }   // loop 'zz_for_action

            // store back cached position
            self.zz_marked_char = zz_marked_char_l;
            self.zz_marked_pos = zz_marked_byte_pos_l;

            if zz_input == Lexer::YYEOF && self.zz_start_read == self.zz_current_pos {
                self.zz_at_eof = true;
                 match self.zz_lexical_state {
                     _ => { println!("normal EOF"); }
                 }

                return Err(Error::EOF);
            } else {
                let action = if zz_action < 0 {
                    zz_action
                } else {
                    Lexer::ZZ_ACTION[zz_action as usize]
                };
                match action {
                    1 => { println!("Token desconocido: {}", self.yytext()); }
                    18 => { /* nothing */ }
                    2 => { println!("Token desconocido: {}", self.yytext()); }
                    19 => { /* nothing */ }
                    3 => { println!("Token desconocido: {}", self.yytext()); }
                    20 => { /* nothing */ }
                    4 => { println!("Token desconocido: {}", self.yytext()); }
                    21 => { /* nothing */ }
                    5 => { println!("Token desconocido: {}", self.yytext()); }
                    22 => { /* nothing */ }
                    6 => { println!("Token desconocido: {}", self.yytext()); }
                    23 => { /* nothing */ }
                    7 => { println!("(CONST, num:{})", self.yytext()); }
                    24 => { /* nothing */ }
                    8 => { println!("(=, -)"); }
                    25 => { /* nothing */ }
                    9 => { println!("((, -)"); }
                    26 => { /* nothing */ }
                    10 => { println!("(), -)"); }
                    27 => { /* nothing */ }
                    11 => { println!("(',', -)"); }
                    28 => { /* nothing */ }
                    12 => { {}; }
                    29 => { /* nothing */ }
                    13 => { println!("(ID, id:{})", self.yytext()); }
                    30 => { /* nothing */ }
                    14 => { println!("(ENDE, -)"); }
                    31 => { /* nothing */ }
                    15 => { println!("(WERT, -)"); }
                    32 => { /* nothing */ }
                    16 => { println!("(FARBE, -)"); }
                    33 => { /* nothing */ }
                    17 => { println!("(ANFANG, -)"); }
                    34 => { /* nothing */ }

                    _ => {
                        return Err(Error::Unmatch);
                    }
                }
            }
        }   // loop
        // never reach end of function
    }

}
