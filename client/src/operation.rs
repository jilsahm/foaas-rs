pub enum Operation<'a> {
    Absolutely { from: &'a str, company: &'a str }
}

impl Operation<'_> {

    pub fn uri(&self) -> String {
        match self {
            &Self::Absolutely { from, company } => format!("/absolutely/{}/{}", company, from),
        }
    }
}