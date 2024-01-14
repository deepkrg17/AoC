use std::fs;

use serde_json::Value;

fn sum(v: Value, skip_red: bool) -> i64 {
    match v {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(vals) => vals.into_iter().map(|v| sum(v, skip_red)).sum(),
        Value::Object(obj) => {
            if skip_red && obj.values().any(|v| v == "red") {
                return 0;
            }
            obj.into_iter().map(|(_, v)| sum(v, skip_red)).sum()
        }
        _ => 0,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let v: Value = serde_json::from_str(&input).unwrap();
    println!("part 1: {}", sum(v.clone(), false));
    println!("part 2: {}", sum(v, true));
}
