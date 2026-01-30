use anyhow::bail;
use serde_json::{Map, Value};
use tracing::{info, warn};

pub struct Json2CsvConverter;

impl Json2CsvConverter {
    pub fn convert(input: &str) -> anyhow::Result<String> {
        let mut csv: String;
        // a sequence
        match serde_json::from_str::<Vec<Map<String, Value>>>(input) {
            Ok(result) => {
                info!("sequence {:?}", result);
                for obj in result {}
                csv = "".into();
            }
            Err(_e) => {
                // single object
                let result = serde_json::from_str::<Map<String, Value>>(input)?;
                csv = result
                    .keys()
                    .map(String::as_str)
                    .collect::<Vec<&str>>()
                    .join(",");

                csv += result
                    .values()
                    .map(|val| match val {
                        Value::Null => Ok("null".to_string()),
                        Value::Bool(b) => Ok(b.to_string()),
                        Value::Number(n) => Ok(n.to_string()),
                        Value::String(s) => Ok(s.clone()),
                        _ => Err(bail!("unsupported!")),
                    })
                    .collect::<anyhow::Result<Vec<String>>>()?
                    .join(",")
                    .as_str();
            }
        }

        Ok(csv)
    }
}
