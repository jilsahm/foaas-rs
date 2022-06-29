use clap::Parser;

#[derive(Debug, Parser)]
pub enum Operation {

    #[clap(help = "Will return content with the current FOAAS version number.")]
    Version,

    #[clap(help = "Will return a JSON list of operations with names and fields. Note: JSON Only")]
    Operations,

    #[clap(help = "Will return content of the form 'Absolutely fucking Not, :company, No Fucking Way! - :from'")]
    Absolutely { #[clap(long)] company: String, #[clap(long)] from: String, },

    #[clap(help = "Will return content of the form 'Fuck you, asshole. - :from'")]
    Asshole { #[clap(long)] from: String, },

    #[clap(help = "Will return content of the form 'This is Fucking Awesome. - :from'")]
    Awesome { #[clap(long)] from: String, },

    #[clap(help = "Will return content of the form ':name, back the fuck off. - :from'")]
    Back { #[clap(long)] name: String, #[clap(long)] from: String, },
    
    #[clap(help = "Will return content of the form 'Eat a bag of fucking dicks. - :from'")]
    Bag { #[clap(long)] from: String, },
    
    #[clap(help = "Will return content of the form 'Fucking :name is a fucking pussy. I'm going to fucking bury that guy, I have done it before, and I will do it again. I'm going to fucking kill :company. - :from'")]
    Ballmer { #[clap(long)] name: String, #[clap(long)] company: String, #[clap(long)] from: String, },

}

impl Operation {

    pub fn uri(&self) -> String {
        match self {
            Self::Version => "version".to_string(),
            Self::Operations => "operations".to_string(),
            Self::Absolutely { from, company } => format!("absolutely/{company}/{from}", company = company, from = from),
            Self::Asshole { from } => format!("asshole/{from}", from = from),
            Self::Awesome { from } => format!("awesome/{from}", from = from),
            Self::Back { name, from } => format!("back/{name}/{from}", name = name, from = from),
            Self::Bag { from } => format!("bag/{from}", from = from),
            Self::Ballmer { name, company, from } => format!("ballmer/{name}/{company}/{from}", name = name, company = company, from = from),
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