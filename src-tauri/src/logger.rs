pub struct Logger;

impl Logger {
    pub fn info<T>(scope: T, message: T) where T : ToString {
        println!("[INFO][{}] {}", scope.to_string(), message.to_string());
    }
    pub fn warn<T>(scope: T, message: T) where T : ToString {
        println!("[WARN][{}] {}", scope.to_string(), message.to_string());
    }
    pub fn error<T>(scope: T, message: T) where T : ToString {
        println!("[ERROR][{}] {}", scope.to_string(), message.to_string());
    }
    pub fn fatal<T>(scope: T, message: T) where T : ToString {
        println!("[FATAL][{}] {}", scope.to_string(), message.to_string());
    }
}
