use tantivy::schema::DateTimePrecision;

#[napi(js_name = "DateTimePrecision")]
pub enum JsDateTimePrecision {
  Seconds,
  Milliseconds,
  Microseconds,
  Nanoseconds,
}

impl From<DateTimePrecision> for JsDateTimePrecision {
  fn from(value: DateTimePrecision) -> Self {
    match value {
      DateTimePrecision::Seconds => Self::Seconds,
      DateTimePrecision::Milliseconds => Self::Milliseconds,
      DateTimePrecision::Microseconds => Self::Microseconds,
      DateTimePrecision::Nanoseconds => Self::Nanoseconds,
    }
  }
}

impl From<JsDateTimePrecision> for DateTimePrecision {
  fn from(value: JsDateTimePrecision) -> Self {
    match value {
      JsDateTimePrecision::Seconds => Self::Seconds,
      JsDateTimePrecision::Milliseconds => Self::Milliseconds,
      JsDateTimePrecision::Microseconds => Self::Microseconds,
      JsDateTimePrecision::Nanoseconds => Self::Nanoseconds,
    }
  }
}
