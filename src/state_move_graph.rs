use std::vec;



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegEx {
    Any,
    Range(char, char),
    Match(char),
    Option(Box<RegEx>),
    Link(Vec<RegEx>),
    Fork(Vec<RegEx>),
}

impl RegEx {
    pub fn optimization(self) -> Self {
        match self {
            RegEx::Link(es) => RegEx::Link(es.into_iter().map(RegEx::optimization).collect()),
            RegEx::Fork(es) => {
                let r: Vec<RegEx> = es.into_iter().map(RegEx::optimization).collect();
                // r cross product
                let mut r_cross_r = vec![];
                for i in 0..r.len() {
                    for j in 0..r.len() {
                        if i != j {
                            r_cross_r.push(RegEx::Fork(vec![r[i].clone(), r[j].clone()]));
                        }
                    }
                }
                todo!()
            },
            _ => self,
        }
    }

    pub fn reduce(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (RegEx::Range(a, b), RegEx::Range(a1, b1)) => if a == a1 && b == b1 {
                Some(RegEx::Range(*a, *b))
            } else {
                None
            },
            (RegEx::Match(a), RegEx::Match(b)) => if a == b {
                Some(RegEx::Match(*a))
            } else {
                None
            },
            (RegEx::Link(es), RegEx::Link(es1)) => {
                let mut rv = vec![];
                for i in es.iter().zip(es1.iter()) {
                    if let Some(e) = i.0.reduce(i.1) {
                        rv.push(RegEx::Link(vec![e]))
                    } else {
                        break;
                    }
                }
                if rv.len() != 0 {
                    rv.push(RegEx::Fork(vec![
                        RegEx::Link(es[rv.len()..].to_vec()),
                        RegEx::Link(es1[rv.len()..].to_vec())]));
                    return Some(RegEx::Link(rv));
                } else {
                    None
                }
            },
            (RegEx::Fork(es), RegEx::Fork(es1)) => if es == es1 {
                Some(RegEx::Fork(es.clone()))
            } else {
                None
            },
            _ => None,
        }
    }
}

impl RegEx {
    pub fn is_match(&self) -> bool {
        match self {
            RegEx::Match(_) => true,
            _ => false,
        }
    }
    pub fn is_range(&self) -> bool {
        match self {
            RegEx::Range(_, _) => true,
            _ => false,
        }
    }
    pub fn is_link(&self) -> bool {
        match self {
            RegEx::Link(_) => true,
            _ => false,
        }
    }
    pub fn is_option(&self) -> bool {
        match self {
            RegEx::Fork(_) => true,
            _ => false,
        }
    }
}