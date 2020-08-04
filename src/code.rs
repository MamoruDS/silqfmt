use std::cmp::Ordering;

pub struct Code {
    raw: String,
    code: String,
    lines: Vec<String>,
    depths: Vec<usize>,
    _depth: usize,
}

impl Code {
    pub fn new(code: String) -> Code {
        Code {
            raw: code,
            code: String::default(),
            lines: vec![String::new()],
            depths: vec![0],
            _depth: 0,
        }
    }

    fn last_depth(&self) -> usize {
        *self.depths.last().unwrap()
    }

    fn newline(&mut self, depth: usize) {
        self.lines.push(String::new());
        self.depths.push(depth);
    }

    fn read(&mut self, c: char) {
        match c {
            '\n' => {
                if self.lines.len() == 1 && self.lines.get(0).unwrap().len() == 0 {
                    return;
                }
                self.newline(self.last_depth());
            }
            '{' => {
                self.newline(self.last_depth() + 1);
            }
            '}' => {
                self.newline(self.last_depth() - 1);
            }
            ' ' => {
                let _line = self.lines.last_mut().unwrap();
                if _line.len() != 0 {
                    _line.push(c);
                }
            }
            _ => {
                self.lines.last_mut().unwrap().push(c);
            }
        }
    }

    pub fn format(&mut self) -> String {
        let code = self.raw.clone();
        for (i, c) in code.chars().enumerate() {
            if i < 100 && i > 90 {
                println!("read c: {}", c);
                println!("curl l: {:?}", self.lines);
            }
            self.read(c);
        }
        self.code = String::new();
        self.format_line(0, 0);
        self.code.clone()
    }

    fn format_line(&mut self, i: usize, last_depth: usize) {
        if i >= self.lines.len() {
            return;
        }
        let depth: usize = *self.depths.get(i).unwrap();
        let content = self.lines.get(i).unwrap();

        match depth.cmp(&last_depth) {
            Ordering::Equal => {
                if self.code.len() != 0 {
                    self.code.push('\n');
                }
                self.code.push_str(&gen_indent(depth));
                self.code.push_str(content);
            }
            Ordering::Greater => {
                self.code.push(' ');
                self.code.push('{');
                match self.depths.get(i + 1) {
                    Some(d) => match d.cmp(&depth) {
                        Ordering::Less => {
                            let c = self.lines.get(i + 1).unwrap();
                            self.code.push('}');
                            self.code.push('\n');
                            self.code.push_str(&gen_indent(*d));
                            self.code.push_str(match c.len() {
                                0 => " ",
                                _ => c,
                            });
                            self.format_line(i + 2, *d);
                            return;
                        }
                        _ => {}
                    },
                    _ => {}
                }
                self.code.push('\n');
                self.code.push_str(&gen_indent(depth));
                self.code.push_str(content);
            }
            Ordering::Less => {
                self.code.push('\n');
                self.code.push_str(&gen_indent(depth));
                self.code.push('}');
                self.code.push('\n');
                self.code.push_str(&gen_indent(depth));
                self.code.push_str(content);
            }
        }
        self.format_line(i + 1, depth)
    }
}

fn gen_indent(depth: usize) -> String {
    let mut indent: String = String::new();
    for _ in 0..depth {
        for _ in 0..4 {
            indent.push(' ')
        }
    }
    indent
}
