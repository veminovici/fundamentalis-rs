use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Command {
    Store(Vec<String>),
    Query,
    QueryResults,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Store(metrics) => {
                write!(f, "STORE {} metrics", metrics.len())
            }
            Command::Query => {
                write!(f, "QUERY")
            }
            Command::QueryResults => {
                write!(f, "QUERY_RESULTS")
            }
        }
    }
}
