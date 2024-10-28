use std::{io::Write, rc::Rc};

#[derive(Clone)]
enum Inner {
    Constant(&'static str),
    String(Rc<str>),
    Join(&'static str, Rc<Vec<Self>>),
    Indent(Rc<Self>),
}

impl Inner {
    fn print(self, indent_size: usize, out: &mut dyn Write) -> anyhow::Result<()> {
        let mut stack = vec![(0, self)];
        let mut spaces = String::new();
        while let Some((indent, next)) = stack.pop() {
            match next {
                Self::Constant(s) => write!(out, "{}{}", &spaces[..indent], s)?,
                Self::String(s) => write!(out, "{}{}", &spaces[..indent], s)?,
                Self::Join(sep, docs) => {
                    for (i, doc) in Rc::unwrap_or_clone(docs).into_iter().rev().enumerate() {
                        if i > 0 {
                            stack.push((indent, Self::Constant(sep)));
                        }
                        stack.push((indent, doc.clone()));
                    }
                }
                Self::Indent(doc) => {
                    (0..indent_size).for_each(|_| spaces.push(' '));
                    stack.push((indent + indent_size, Rc::unwrap_or_clone(doc)));
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Doc {
    inner: Inner,
}

impl Doc {
    pub fn constant(data: &'static str) -> Self {
        Self {
            inner: Inner::Constant(data),
        }
    }

    pub fn string(data: String) -> Self {
        Self {
            inner: Inner::String(data.into()),
        }
    }

    pub fn join(sep: &'static str, docs: Vec<Self>) -> Self {
        Self {
            inner: Inner::Join(sep, Rc::new(docs.into_iter().map(|x| x.inner).collect())),
        }
    }

    pub fn then(self, other: Self) -> Self {
        Self::join("", vec![self, other])
    }

    pub fn indent(self) -> Self {
        Self {
            inner: Inner::Indent(Rc::new(self.inner)),
        }
    }

    pub fn print(self, indent_size: usize, out: &mut dyn Write) -> anyhow::Result<()> {
        self.inner.print(indent_size, out)
    }
}

impl From<&'static str> for Doc {
    fn from(value: &'static str) -> Self {
        Self::constant(value)
    }
}
