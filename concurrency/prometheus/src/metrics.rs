use std::str::FromStr;
use log::error;


#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub timestamp: i64,
    pub value: f64,
}

impl FromStr for Metric {
    type Err = i8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#") {
            return Err(0)
        }
        // if s.contains("{") || s.contains("") {
        //     return Err(0)
        // }

        let mut iter = s.split_whitespace();
        let name = iter.next();
        let value = iter.next();

        if name.is_none() || value.is_none() {
            return Err(1)
        }

        let pvalue = value.unwrap().parse::<f64>();
        if pvalue.is_err() {
            error!("failed to parse {}", name.unwrap());
            return Err(2)
        }

        let m = Metric {
            name: name.unwrap().to_string(),
            timestamp: 0,
            value: pvalue.unwrap(),
        };
        
        Ok(m)
    }
}

impl Into<(i64, f64)> for Metric {
    fn into(self) -> (i64, f64) {
        (self.timestamp, self.value)
    }
}