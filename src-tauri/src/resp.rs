use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct R<T> {
    pub code: i64,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> From<Result<T>> for R<T>
where
    T: Serialize,
{
    fn from(item: Result<T>) -> Self {
        match item {
            Ok(data) => Self::success(data),
            Err(e) => Self::fail(1, &e.to_string()),
        }
    }
}

impl<T> R<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        R {
            code: 0,
            msg: Some("success".to_string()),
            data: Some(data),
        }
    }

    pub fn fail(code: i64, e: &str) -> Self {
        R {
            code,
            msg: Some(e.to_owned()),
            data: None,
        }
    }
}
