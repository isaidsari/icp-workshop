use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref STATUS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    
    // check if starts with get or set
    if name.starts_with("get:") {
        let path = name.split_at(4).1;
        return get_status(path.to_string());
    } else if name.starts_with("set:") {
        set_status(name);
        return "Status set".to_string();
    } else {
        return "Invalid command".to_string();
    }

}

fn get_status(path: String) -> String {
    match STATUS.lock().unwrap().get(&path) {
        Some(status) => status.clone(),
        None => "Not found".to_string(),
    }
}

fn set_status(data: String) {
    let mut parts = data.splitn(2, ':');
    let path = parts.next().unwrap().to_string();
    let status = parts.next().unwrap().to_string();
    STATUS.lock().unwrap().insert(path, status);
}
