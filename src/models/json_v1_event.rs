/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.5.0-master.17
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonV1Event : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonV1Event {
    /// 
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// 
    #[serde(rename = "firstTimestamp", skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<f32>,
    #[serde(rename = "related", skip_serializing_if = "Option::is_none")]
    pub related: Option<crate::models::JsonV1ObjectReference>,
    /// 
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::JsonV1ObjectMeta>,
    /// 
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f32>,
    /// 
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// 
    #[serde(rename = "reportingInstance", skip_serializing_if = "Option::is_none")]
    pub reporting_instance: Option<String>,
    /// 
    #[serde(rename = "reportingComponent", skip_serializing_if = "Option::is_none")]
    pub reporting_component: Option<String>,
    /// 
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<f32>,
    /// 
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::JsonV1EventSource>,
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Option<crate::models::JsonV1EventSeries>,
    /// 
    #[serde(rename = "lastTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<f32>,
    /// 
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "involvedObject", skip_serializing_if = "Option::is_none")]
    pub involved_object: Option<crate::models::JsonV1ObjectReference>,
    /// 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl JsonV1Event {
    /// 
    pub fn new() -> JsonV1Event {
        JsonV1Event {
            message: None,
            first_timestamp: None,
            related: None,
            action: None,
            metadata: None,
            event_time: None,
            api_version: None,
            reporting_instance: None,
            reporting_component: None,
            count: None,
            kind: None,
            source: None,
            series: None,
            last_timestamp: None,
            reason: None,
            involved_object: None,
            _type: None,
        }
    }
}


