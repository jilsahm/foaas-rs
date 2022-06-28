use clap::Parser;

#[derive(Debug, Parser)]
pub enum Operation {

    #[clap(help = "Will return content with the current FOAAS version number.")]
    Version,

    #[clap(help = "Will return a JSON list of operations with names and fields. Note: JSON Only")]
    Operations,

    #[clap(help = "Will return content of the form 'Absolutely fucking Not, :company, No Fucking Way! - :from'")]
    Absolutely { #[clap(long)] company: String, #[clap(long)] from: String,  }
}

impl Operation {

    pub fn uri(&self) -> String {
        match self {
            Self::Version => "version".to_string(),
            Self::Operations => "operations".to_string(),
            Self::Absolutely { from, company } => format!("absolutely/{}/{}", company, from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Operation;


    #[test]
    fn uri() {
        let operation = Operation::Absolutely { company: "company".to_string(), from: "from".to_string() };
        let expected = "absolutely/company/from";
        assert_eq!(expected, operation.uri());
    }
}