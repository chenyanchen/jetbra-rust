use clap::ValueEnum;
use enum_iterator::{all, Sequence};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Sequence, ValueEnum)]
pub enum Apps {
    Idea,
    Clion,
    Goland,
}

impl Apps {
    pub fn all() -> Vec<Apps> {
        all::<Apps>().collect()
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub short: String,
    pub code: String,
}

impl From<Apps> for App {
    fn from(app: Apps) -> Self {
        match app {
            Apps::Idea => App {
                name: "IntelliJ IDEA".into(),
                short: "idea".into(),
                code: "IIU".into(),
            },
            Apps::Clion => App {
                name: "CLion".into(),
                short: "clion".into(),
                code: "CL".into(),
            },
            Apps::Goland => App {
                name: "GoLand".into(),
                short: "goland".into(),
                code: "GO".into(),
            },
        }
    }
}
