use serde::{Deserialize, Serialize};
use std::os::unix::fs::FileTypeExt;

const CURRENT_VERSION: &str = "0.8.0";

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    pub version: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    pub devices: Vec<Device>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_edits: Option<ContainerEdits>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    pub container_edits: ContainerEdits,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ContainerEdits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_nodes: Option<Vec<DeviceNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Vec<Hook>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intel_rdt: Option<IntelRdt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_gids: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceNode {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<u32>, // Use u32 to represent FileMode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    pub host_path: String,
    pub container_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hook {
    pub hook_name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntelRdt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clos_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l3_cache_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_bw_schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cmt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_mbm: Option<bool>,
}
