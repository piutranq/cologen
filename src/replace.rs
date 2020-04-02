use std::io;
use std::io::prelude::*;
use crate::scheme::Scheme;
use regex::Regex;

pub struct Replacer {
    scheme: Scheme,
    state_fsm: usize,
    state_colorname: String,
    state_format: char,
}

impl Replacer {
    pub fn new(path: String) -> Replacer {
        Replacer {
            scheme: Scheme::from_path(path).unwrap(),
            state_fsm: 0,
            state_colorname: String::new(),
            state_format: 'd',
        }
    }

    pub fn execute(&mut self) {
        for line in io::stdin().lock().lines() {
            println!("{}", self.replace_line(line.unwrap()));
        }
    }

    fn init(&mut self) {
        self.state_fsm = 0;
        self.state_colorname = String::new();
        self.state_format = 'd';
    }

    fn replace_line(&mut self, line: String) -> String {
        let mut new = String::new();

        for c in line.chars() {
            match self.execute_fsm(c) {
                Some(c) => new.push_str(&c),
                None => {},
            };
        }
        new
    }

    fn goto(&mut self, num: usize) {
        self.state_fsm = num
    }

    fn set_format(&mut self, format: char) {
        self.state_format = format
    }

    fn execute_fsm(&mut self, c: char) -> Option<String> {
        match self.state_fsm {
            /* State 0: Initial State, waiting for '@'
             *  '@'         : Go to the state 1
             *  The others  : Pass without replace
             */
            0 => {
                match c {
                    '@' => {
                        self.goto(1);
                        None
                    },
                    _ => Some(c.to_string())
                }
            },
            /* State 1: When '@' has parsed, waiting for '['
             *  '@'         : Pass @, and go to the state 0
             *  '['         : Go to the state 2
             * The others   : Unreachable
             */
            1 => {
                match c {
                    '@' => {
                        self.goto(0);
                        Some('@'.to_string())
                    },
                    '[' => {
                        self.goto(2);
                        None
                    },
                    _ => unreachable!()
                }
            },
            /* State 2: When '@[' have parsed, parsing the color name
             *          and waiting for ':'
             *  ':'                         : Go to the state 3
             *  '[A-Z]|[a-z]|[0-9]|_' : Push to the color term
             *  The others                  : Unreachable
            */
            2 => {
                let re = Regex::new(r"[A-Z]|[a-z]|[0-9]|_").unwrap();
                match c {
                    ':' => {
                        self.goto(3);
                        None
                    },
                    _ => {
                        match re.captures(&(format!("{}", c))) {
                            Some(_) => self.state_colorname.push(c),
                            None => unreachable!()
                        };
                        None
                    }
                }
            },
            /* State 3: When '@[' and ':' have parsed,
             *          parsing the format and waiting ']'
             *  '%'         : Go to the state 4
             *  ']'         : Go to the state 0, and initialize all states
             *  The others  : Pass without replace
             */
            3 => {
                match c {
                    '%' => {
                        self.goto(4);
                        None
                    },
                    ']' => {
                        self.init();
                        None
                    },
                    _ => Some(c.to_string())
                }
            },
            /* State 4: When '%' has parsed in the state 3,
             *          waiting for format character
             *  'd|p|.|x'   : Go to the state 5
             *                and set format as formated character.
             *  '%'         : Pass '%', and go to the state 3
             *  The others  : Unreachable
             */
            4 => {
                match c {
                    'd'|'p'|'.'|'x' => {
                        self.goto(5);
                        self.set_format(c);
                        None
                    },
                    '%' => {
                        self.goto(3);
                        Some(c.to_string())
                    }
                    _   => unreachable!()
                }
            },
            /* State 5: When the format character has parsed in the state 4,
             *          waiting for offset character and pass color value
             *  'R|G|B|A'   : Pass the R|G|B|A value and go to the state 3
             *  The others  : Unreachable
             */
            5 => {
                let color = self.scheme.color(self.state_colorname.clone());
                let value = color.value(self.state_format, c);
                self.goto(3);
                Some(value)
            },
            _ => unreachable!()
        }
    }

}
