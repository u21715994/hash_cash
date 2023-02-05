fn main() {
    let data = r#"{
    "Welcome":{
        "version":1
        }
    }"#;

    let v: Value = serde_json::from_str(data)?;
}