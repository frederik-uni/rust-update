use std::fmt::Display;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Version(u64, u64, u64);

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Next {
    a: u64,
    b: u64,
    next: Option<Box<Next>>,
}

fn compare(a: u64, b: u64, next: Option<Next>) -> std::cmp::Ordering {
    let ordering = a.cmp(&b);
    match ordering {
        std::cmp::Ordering::Equal => match next {
            Some(v) => compare(v.a, v.b, v.next.map(|v| *v)),
            None => ordering,
        },
        _ => ordering,
    }
}
impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let next = Next {
            a: self.1,
            b: other.1,
            next: Some(Box::new(Next {
                a: self.2,
                b: other.2,
                next: None,
            })),
        };
        compare(self.0, other.0, Some(next))
    }
}

impl TryFrom<&str> for Version {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut v = value.split(".");
        let major = v.next().ok_or(())?.parse().map_err(|_| ())?;
        let minor = v.next().ok_or(())?.parse().map_err(|_| ())?;
        let patch = v.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Self::new(major, minor, patch))
    }
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self(major, minor, patch)
    }
}
