use chrono::prelude::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metric {
    ts: u64,
    app_name: String,
    app_layer: String,
    value: f32,
    operation: Option<String>,
    metric_name: String,
}

impl Metric {
    pub fn to_val(&self) -> Vec<String> {
        vec![format!("'{}'", self.metric_name),
             format!("{}", self.value),
             format!("'{}'", NaiveDateTime::from_timestamp(self.ts as i64, 0)),
             format!("'{}'", self.app_name),
             format!("'{}'", self.app_layer),
             format!("'{}'", self.operation.as_ref().unwrap_or(&String::new()))]
    }
}
