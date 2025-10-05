use substring::Substring;

pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Continue,
    Backtrace,
    Break(String),
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            },
            "c" | "cont" => Some(DebuggerCommand::Continue),
            "bt" | "back" | "backtrace" => Some(DebuggerCommand::Backtrace),
            "b" | "break" => { 
                let str = tokens[1].to_string();
                assert!(str.substring(0, 1) == "*");
                Some(DebuggerCommand::Break(str.substring(1, str.len()).to_string()))
            },
            // Default case:
            _ => None,
        }
    }
}
