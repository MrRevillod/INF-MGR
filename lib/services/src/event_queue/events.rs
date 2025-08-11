use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Event {
    PracticeCreated(Value),
    PracticeApproved(Value),
    UserCreated(Value),
}
