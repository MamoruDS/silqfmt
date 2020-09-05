use regex::Regex;
use std::{cmp::Ord, collections::HashMap, fmt, ops::Range};

struct RegexMatch {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub policy: i8,
}

impl fmt::Debug for RegexMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Regex Match {{ text: {}, start: {}, end: {}, policy: {} }}",
            self.text, self.start, self.end, self.policy,
        )
    }
}

pub struct PatternRec {
    matched: Vec<RegexMatch>,
    code: String,
}

impl PatternRec {
    pub fn new(code: &str) -> PatternRec {
        PatternRec {
            matched: Vec::new(),
            code: String::from(code),
        }
    }

    fn is_matched(&self, start: usize, end: usize) -> bool {
        for m in self.matched.iter() {
            if start > m.end || end < m.start {
                continue;
            } else {
                return true;
            }
        }
        return false;
    }

    pub fn find(&mut self, re: &str, group_range: Range<usize>, group_policy: HashMap<i8, i8>) {
        let re = Regex::new(re).unwrap();
        let mut _found = 0;
        for cap in re.captures_iter(&self.code[..]) {
            let text: &str;
            let start: usize;
            let end: usize;
            match &cap.name("body") {
                Some(mat) => {
                    start = mat.start();
                    end = mat.end();
                    text = mat.as_str();
                }
                None => continue,
            };

            if self.is_matched(start, end) {
                continue;
            }

            let mut group_num: i8 = 0;
            for g in group_range.start..group_range.end {
                match cap.get(g) {
                    Some(_) => {
                        group_num = g as i8;
                    }
                    _ => {}
                }
            }

            self.matched.push(RegexMatch {
                text: String::from(text),
                start: start,
                end: end,
                policy: *group_policy.get(&group_num).unwrap(),
            });
            _found += 1;
        }

        self.sort();
        // println!("found: {} matches.\n{:?}", _found, self.matched);
    }

    pub fn sort(&mut self) {
        self.matched.sort_by(|a, b| a.start.cmp(&b.start));
    }
}
