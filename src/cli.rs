#[derive(Debug, clap::Parser)] // requires `derive` feature
#[command(name = "clc")]
#[command(about = "persistent calculator", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Add {
        #[arg(default_value = "1.0")]
        value: f64,
    },
    Sub {
        #[arg(default_value = "1.0")]
        value: f64,
    },
    Mul {
        value: f64,
    },
    Div {
        value: f64,
    },
    Set {
        new_value: f64,
    },
    Switch {
        name: String,
    },
    New {
        name: String,
        #[arg(default_value = "0.0")]
        value: f64,
        #[arg(default_value = "false")]
        overwrite: Bool,
    },
    List,
    // History,
    // States,
    // NameState,
    PrintTiming {
        value: Option<bool>,
    },
    Delete {
        name: String,
    },
    DeleteAll,
}

#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Bool {
    True,
    False,
}
