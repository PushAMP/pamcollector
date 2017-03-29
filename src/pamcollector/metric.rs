use std::collections::HashMap;
use chrono::prelude::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metric {
    ts: u64,
    app_name: String,
    app_layer: String,
    value: f32,
    operation: Option<String>,
    metric_name: String,
    labels: Option<HashMap<String, String>>,
}

impl Metric {
    pub fn to_val(&self) -> Vec<String> {
        let (labels, labels_val) = match self.labels {
            Some(ref v) => {
                let mut vec1 = Vec::with_capacity(v.len());
                let mut vec2 = Vec::with_capacity(v.len());
                for (k, v) in v.iter() {
                    vec1.push(k);
                    vec2.push(v);
                }
                println!("{:?} {:?}", vec1, vec2);
                (vec1, vec2)
            }
            None => (Vec::new(), Vec::new()),
        };
        println!("{:?} {:?}", labels, labels_val);
        vec![format!("'{}'", self.metric_name),
             format!("{}", self.value),
             format!("'{}'", NaiveDateTime::from_timestamp(self.ts as i64, 0)),
             format!("'{}'", self.app_name),
             format!("'{}'", self.app_layer),
             format!("'{}'", self.operation.as_ref().unwrap_or(&String::new()))]
    }
}

