use std::{collections::HashMap, fmt};

#[derive(Debug)]
struct Policy<'t> {
    prefix: Option<&'t str>,
    prefix_round_overwrite: Option<&'t str>,
    prefix_curly_overwrite: Option<&'t str>,
    prefix_square_overwrite: Option<&'t str>,
    suffix: Option<&'t str>,
    suffix_round_overwrite: Option<&'t str>,
    suffix_curly_overwrite: Option<&'t str>,
    suffix_square_overwrite: Option<&'t str>,
}

impl<'t> Policy<'t> {
    fn new() -> Policy<'t> {
        Policy {
            prefix: None,
            prefix_round_overwrite: None,
            prefix_curly_overwrite: None,
            prefix_square_overwrite: None,
            suffix: None,
            suffix_round_overwrite: None,
            suffix_curly_overwrite: None,
            suffix_square_overwrite: None,
        }
    }
}

pub const KEYWORD_NORMAL: i8 = 0;
pub const KEYWORD_HEAD: i8 = 1;

pub struct FormatPolicy<'t> {
    policies: HashMap<i8, Policy<'t>>,
}

impl<'t> FormatPolicy<'t> {
    pub fn new() -> FormatPolicy<'t> {
        let mut p = FormatPolicy {
            policies: HashMap::new(),
        };

        p.policies.insert(KEYWORD_NORMAL, {
            let mut _p = Policy::new();
            _p.prefix = Some(&" ");
            _p.suffix = Some(&" ");
            _p
        });

        p.policies.insert(KEYWORD_HEAD, {
            let mut _p = Policy::new();
            _p.prefix = Some(&"\n");
            _p.prefix_round_overwrite = None;
            _p.suffix = Some(&" ");
            _p
        });

        p
    }
}
