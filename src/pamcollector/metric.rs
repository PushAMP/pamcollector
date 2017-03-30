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
        let labels_str = format!("[{}]",
                                 labels.iter()
                                     .map(|x| format!("'{}'", x))
                                     .collect::<Vec<_>>()
                                     .join(", "));
        let labels_var_str = format!("[{}]",
                                     labels_val.iter()
                                         .map(|x| format!("'{}'", x))
                                         .collect::<Vec<_>>()
                                         .join(", "));
        println!("{}, {}", labels_str, labels_var_str);
        vec![format!("'{}'", self.metric_name),
             format!("{}", self.value),
             format!("'{}'", NaiveDateTime::from_timestamp(self.ts as i64, 0)),
             format!("'{}'", self.app_name),
             format!("'{}'", self.app_layer),
             format!("'{}'", self.operation.as_ref().unwrap_or(&String::new())),
             format!("{}", labels_str),
             format!("{}", labels_var_str)]
    }

    pub fn clean_labels(&mut self) {
        let ignore_labels = vec!["app_name", "app_layer", "value", "operation", "metric_name"];
        match self.labels.as_mut() {
            Some(v) => v.retain(|ref k, _| !ignore_labels.contains(&k.as_str())),
            None => (),
        };
    }
}

