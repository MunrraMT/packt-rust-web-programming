use std::fmt;

use serde::ser::SerializeStruct;

#[derive(Clone)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }

    pub fn from_string(input_string: String) -> Result<Self, String> {
        match input_string.as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(format!("input {input_string} not supported")),
        }
    }
}

impl serde::Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("TaskStatus", 1)?;
        s.serialize_field("status", &self.stringify())?;
        s.end()
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {
                write!(f, "DONE")
            }

            &Self::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}
