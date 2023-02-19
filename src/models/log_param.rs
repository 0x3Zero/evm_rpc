use ethabi::EventParam;
use marine_rs_sdk::marine;

#[marine]
#[derive(Debug)]
pub struct EventLogParamResult {
    pub event_name: String,
    pub params: Vec<DataLogParam>,
    pub success: bool,
    pub error_msg: String,
}

#[marine]
#[derive(Debug)]
pub struct DataLogParam {
    pub name: String,
    pub kind: String,
    pub value: String,
}

impl From<EventParam> for DataLogParam {
    fn from(param: EventParam) -> Self {
        Self {
            name: param.name,
            kind: param.kind.to_string(),
            value: "".to_string(),
        }
    }
}
