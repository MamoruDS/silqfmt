use regex::{Captures, Regex};
use std::{collections::HashMap, fmt};

fn capfn_other(cap: &Captures) -> (String, Option<String>) {
    let capture = format!("{}", cap.get(0).map_or("", |m| m.as_str()));
    (capture, None)
}

fn capfn_keyword(cap: &Captures) -> (String, Option<String>) {
    let mut capture: String = String::new();
    for g in 3..6 {
        match cap.get(g) {
            Some(m) => {
                capture = match g {
                    4 => format!(" {} ", m.as_str()),
                    5 => format!("\n{} ", m.as_str()),
                    _ => format!("{}", m.as_str()),
                };
                break;
            }
            _ => continue,
        }
    }
    let mark = format!(
        "{}[#keyword]{}",
        cap.get(1).map_or("", |m| m.as_str()),
        cap.get(6).map_or("", |m| m.as_str())
    );
    (capture, Some(mark))
}

fn capfn_operator(cap: &Captures) -> (String, Option<String>) {
    let mut capture: String = String::new();
    match cap.get(1) {
        Some(m) => capture = format!(" {} ", m.as_str()),
        _ => {}
    }
    (capture, None)
}

fn capfn_terminator(_cap: &Captures) -> (String, Option<String>) {
    (String::from(";\n"), None)
}

fn capfn_comment(cap: &Captures) -> (String, Option<String>) {
    let mut text: String = String::new();
    for g in 1..cap.len()  {
        match cap.get(g) {
            Some(m) => {
                match g {
                    3 => {
                        text.push_str(m.as_str());
                        text.push('\n');
                    }
                    _ => {
                        text.push_str(m.as_str());
                    }
                }
                break;
            }
            _ => {}
        }
    }
    (text, None)
}

pub struct SilqPattern {
    // re: Regex,
    patterns: HashMap<String, (String, fn(&Captures) -> (String, Option<String>))>,
    code: String,
}

impl SilqPattern {
    pub fn new() -> SilqPattern {
        let mut patterns: HashMap<String, (String, fn(&Captures) -> (String, Option<String>))> =
            HashMap::new();
        patterns.insert(
            String::from("comment"),
            (
                String::from(r"(/\*.*?\*/)|((?m)/\*[\s\S]*?\*/)|(//.*)"),
                capfn_comment,
            ),
        );
        patterns.insert(
            String::from("whitespace_all"),
            (String::from(r"(\s)|(\n)"), capfn_other),
        );
        patterns.insert(
            String::from("whitespace_safe"),
            (String::from(r"(?m)\s+$"), capfn_other),
        );
        patterns.insert(
            String::from("terminator"),
            (String::from(r";"), capfn_terminator),
        );
        patterns.insert(
            String::from("keyword"),
            (String::from(r"([^\w|_])((true|false|forget|λ|lambda|observe|cobserve|Π|Pi)|(dat|if|then|else|assert|repeat|in|quantum|lifted|qfree|mfree|as|coerce)|(const|def|for|import|return|while))([^\w|_])"), capfn_keyword),
        );
        patterns.insert(
            String::from("operator"),
            (
                String::from(r"(:=|==|=|>|<|>=|<=|!=|\|\||\&\&|\+|\-|\*|/)"),
                capfn_operator,
            ),
        );

        SilqPattern {
            patterns: patterns,
            code: String::new(),
        }
    }

    pub fn load(&mut self, code: &str) {
        self.code = String::from(code);
        self.code.insert(0, ' ');
    }

    pub fn cache(&mut self, pattern_name: &str) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();

        let (re_str, capfn) = &self.patterns.get(pattern_name).unwrap();
        let re = Regex::new(&re_str[..]).unwrap();
        self.code = String::from(re.replace_all(&self.code, |caps: &Captures| {
            let (capture, mark) = capfn(&caps);
            vec.push(capture);
            match mark {
                Some(m) => m,
                _ => format!("[#{}]", &pattern_name),
            }
        }));
        return vec;
    }

    pub fn remove(&mut self, pattern_name: &str) {
        let (re_str, _) = &self.patterns.get(pattern_name).unwrap();
        let re = Regex::new(&re_str[..]).unwrap();
        self.code = String::from(re.replace_all(&self.code, ""));
    }

    pub fn restore(&mut self, pattern_name: &str, data: &Vec<String>) {
        let re = Regex::new(&format!(r"\[#{}\]", pattern_name)[..]).unwrap();
        let c = self.code.clone();
        let v: Vec<&str> = re.split(&c[..]).collect();
        self.code = String::new();
        for (i, text) in v.iter().enumerate() {
            self.code.push_str(text);
            self.code.push_str(data.get(i).unwrap_or(&String::from("")));
        }
    }
}

impl fmt::Display for SilqPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

pub fn code_fmt(code: &str) -> String {
    let mut p = SilqPattern::new();
    p.load(&code);
    p.remove(&"whitespace_safe");
    // p.remove(&"comment");
    let cache_comments = p.cache(&"comment");
    let cache_keywords = p.cache(&"keyword");
    let cache_operators = p.cache(&"operator");
    let cache_terminators = p.cache(&"terminator");
    p.remove(&"whitespace_all");

    p.restore(&"keyword", &cache_keywords);
    p.restore(&"operator", &cache_operators);
    p.restore(&"terminator", &cache_terminators);
    p.restore(&"comment", &cache_comments);

    p.code
}
