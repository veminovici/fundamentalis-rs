use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Command {
    Store,
    Query,
    QueryResults,
}


impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Store => {
                writeln!(f, "STORE")
            },
            Command::Query => {
                writeln!(f, "QUERY")
            },
            Command::QueryResults => {
                writeln!(f, "QUERY_RESULTS")
            }
        }
    }
}