#[derive(Clone, PartialEq)]
pub enum NotificationType {
    SuccessAdd,
    SuccessUpdate,
    SuccessDelete,
    Error(String),
}

#[derive(Clone, Default)]
pub struct UpdateForm {
    pub title: String,
    pub description: String,
    pub due_date: String,
}
