use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};

use cpp::*;
use thiserror::Error;

cpp! {{
    #include <string.h>
    #include "tflite/public/edgetpu.h"
    
     struct NoDelete {
        void operator()(edgetpu::EdgeTpuManager* ptr) const {
            // Do nothing
        }
    };

    extern "C" std::shared_ptr<edgetpu::EdgeTpuManager> get_edgetpu_manager_singleton() {
        return std::shared_ptr<edgetpu::EdgeTpuManager>(edgetpu::EdgeTpuManager::GetSingleton(), NoDelete());
    }
}}

#[derive(Debug, Error)]
pub enum EdgeTPUError {
    #[error("failed to open device")]
    OpenFailed,
    #[error("failed to set verbosity")]
    SetVerbosityFailed,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum DeviceType {
    ApexPCI,
    ApexUSB,
}

#[derive(PartialEq, Clone, Debug)]
pub struct DeviceRecord {
    pub device_type: DeviceType,
    pub path: String,
}

cpp_class!(unsafe struct InnerEdgeTpuContext as "std::shared_ptr<edgetpu::EdgeTpuContext>");
cpp_class!(unsafe struct InnerEdgeTpuDeviceManager as "std::shared_ptr<edgetpu::EdgeTpuManager>");
cpp_class!(unsafe struct EdgeTpuDeviceType as "edgetpu::DeviceType");
cpp_class!(unsafe struct EdgeTpuDeviceOptions as "edgetpu::EdgeTpuManager::DeviceOptions");

#[derive(Clone)]
pub struct EdgeTpuContext {
    inner: Arc<Mutex<InnerEdgeTpuContext>>
}

impl EdgeTpuContext {
    pub fn is_ready(&self) -> bool {
        let inner = self.inner.lock().unwrap().clone();
        let ok = cpp!(unsafe [inner as "std::shared_ptr<edgetpu::EdgeTpuContext>"] -> i32 as "int" {
            return !!inner->IsReady();
        });
        ok != 0 
    }

    pub fn get_device_enum_record(&self) -> Result<DeviceRecord, EdgeTPUError> {
        let inner = self.inner.lock().unwrap().clone();
        let (device_type, path) = cpp!(unsafe [inner as "std::shared_ptr<edgetpu::EdgeTpuContext>"] -> (u8, *mut c_char) as "std::tuple<char, const char*>" {
            const auto& device = inner->GetDeviceEnumRecord();
            std::string path(device.path);
            char device_type;
            if (device.type == edgetpu::DeviceType::kApexPci) {
                device_type = 0;
            } else if (device.type == edgetpu::DeviceType::kApexUsb) {
                device_type = 1;
            }
            return std::make_tuple(device_type, strdup(path.c_str()));
        });

        let path = unsafe { CString::from_raw(path).to_string_lossy().into_owned() };
        let device_type = match device_type {
            0 => DeviceType::ApexPCI,
            1 => DeviceType::ApexUSB,
            _ => return Err(EdgeTPUError::OpenFailed),
        };
        Ok(DeviceRecord { device_type, path })
    }

    pub fn device_options(&self) -> HashMap<String, String> {
        let inner = self.inner.lock().unwrap().clone();
        let mut options = HashMap::new();
        let options_ptr = &mut options;
        cpp!(unsafe [inner as "std::shared_ptr<edgetpu::EdgeTpuContext>", options_ptr as "void *"] {
            auto opts = inner->GetDeviceOptions();
            for (const auto& element : opts) {
                const char* key = element.first.c_str();
                const char* value = element.second.c_str();

                rust!(edgetpu_device_options_cb [options_ptr: *mut HashMap<String, String> as "void *", key: *const c_char as "const char *", value: *const c_char as "const char *"] {
                    let key = unsafe { CStr::from_ptr(key) }.to_string_lossy().into_owned();
                    let value = unsafe { CStr::from_ptr(value) }.to_string_lossy().into_owned();
                    options_ptr.as_mut().unwrap().insert(key, value);
                });
            }
        });
        options
    }
}

#[derive(Clone)]
pub struct EdgeTpuDeviceManager {
    inner: Arc<Mutex<InnerEdgeTpuDeviceManager>>, // Use Arc<Mutex<_>> to simulate shared_ptr behavior
}

impl EdgeTpuDeviceManager {
    pub fn get_singleton() -> Option<EdgeTpuDeviceManager> {
        let inner = cpp!(unsafe [] -> InnerEdgeTpuDeviceManager as "std::shared_ptr<edgetpu::EdgeTpuManager>" {
            return get_edgetpu_manager_singleton();
        });

        Some(EdgeTpuDeviceManager {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub fn enumerate_devices(&self) -> Vec<DeviceRecord> {
        let mut devices = Vec::new();
        let devices_ptr = &mut devices;

        cpp!(unsafe [devices_ptr as "void *"] {
            const auto& available_tpus = get_edgetpu_manager_singleton()->EnumerateEdgeTpu();
            for (const auto& device : available_tpus) {
                std::string device_path = device.path;
                int device_type;
                if (device.type == edgetpu::DeviceType::kApexPci) {
                    device_type = 0;
                } else if (device.type == edgetpu::DeviceType::kApexUsb) {
                    device_type = 1;
                }

                rust!(enumerate_devices_cb [devices_ptr: *mut Vec<DeviceRecord> as "void *", device_path: *mut c_char as "std::string", device_type: u8 as "char"] {
                    let path = unsafe { CStr::from_ptr(device_path) }.to_string_lossy().into_owned();
                    let device_type = match device_type {
                        0 => DeviceType::ApexPCI,
                        1 => DeviceType::ApexUSB,
                        _ => panic!("unresolved device type"),
                    };
                    devices_ptr.as_mut().unwrap().push(DeviceRecord{
                        device_type,
                        path: path,
                    });
                });
            }
        });

        devices
    }

    pub fn open_device(&self) -> Result<EdgeTpuContext, EdgeTPUError> {
        let inner = cpp!(unsafe [] -> InnerEdgeTpuContext as "std::shared_ptr<edgetpu::EdgeTpuContext>" {
            return get_edgetpu_manager_singleton()->OpenDevice();
        });

        let ok = cpp!(unsafe [inner as "std::shared_ptr<edgetpu::EdgeTpuContext>"] -> i32 as "int" {
            return !!inner;
        });

        match ok {
            0 => Err(EdgeTPUError::OpenFailed),
            _ => Ok(EdgeTpuContext {
                inner: Arc::new(Mutex::new(inner)),
            }),
        }
    }

    pub fn open_device_type(&self, device_type: DeviceType) -> Result<EdgeTpuContext, EdgeTPUError> {
        let device_type = match device_type {
            DeviceType::ApexPCI => cpp!(unsafe [] -> EdgeTpuDeviceType as "edgetpu::DeviceType" {
                return edgetpu::DeviceType::kApexPci;
            }),
            DeviceType::ApexUSB => cpp!(unsafe [] -> EdgeTpuDeviceType as "edgetpu::DeviceType" {
                return edgetpu::DeviceType::kApexUsb;
            }),
        };

        let inner = cpp!(unsafe [device_type as "edgetpu::DeviceType"] -> InnerEdgeTpuContext as "std::shared_ptr<edgetpu::EdgeTpuContext>" {
            return get_edgetpu_manager_singleton()->OpenDevice(device_type);
        });

        let ok = cpp!(unsafe [inner as "std::shared_ptr<edgetpu::EdgeTpuContext>"] -> i32 as "int" {
            return !!inner;
        });

        match ok {
            0 => Err(EdgeTPUError::OpenFailed),
            _ => Ok(EdgeTpuContext {
                inner: Arc::new(Mutex::new(inner)),
            }),
        }
    }

    pub fn open_device_path(&self, device_type: DeviceType, path: &str) -> Result<EdgeTpuContext, EdgeTPUError> {
        let device_type = match device_type {
            DeviceType::ApexPCI => cpp!(unsafe [] -> EdgeTpuDeviceType as "edgetpu::DeviceType" {
                return edgetpu::DeviceType::kApexPci;
            }),
            DeviceType::ApexUSB => cpp!(unsafe [] -> EdgeTpuDeviceType as "edgetpu::DeviceType" {
                return edgetpu::DeviceType::kApexUsb;
            }),
        };

        let path_s = CString::new(path).unwrap();
        let path = path_s.as_ptr();

        let inner = cpp!(unsafe [device_type as "edgetpu::DeviceType", path as "const char *"] -> InnerEdgeTpuContext as "std::shared_ptr<edgetpu::EdgeTpuContext>" {
            auto device_path = std::string(path);
            return get_edgetpu_manager_singleton()->OpenDevice(device_type, device_path);
        });

        let ok = cpp!(unsafe [inner as "std::shared_ptr<edgetpu::EdgeTpuContext>"] -> i32 as "int" {
            return !!inner;
        });

        match ok {
            0 => Err(EdgeTPUError::OpenFailed),
            _ => Ok(EdgeTpuContext {
                inner: Arc::new(Mutex::new(inner)),
            }),
        }
    }

    pub fn open_device_options(
        &self,
        device_type: DeviceType,
        path: &str,
        options: HashMap<String, String>,
    ) -> Result<EdgeTpuContext, EdgeTPUError> {
        let device_type = match device_type {
            DeviceType::ApexPCI => cpp!(unsafe [] -> EdgeTpuDeviceType as "edgetpu::DeviceType" {
                return edgetpu::DeviceType::kApexPci;
            }),
            DeviceType::ApexUSB => cpp!(unsafe [] -> EdgeTpuDeviceType as "edgetpu::DeviceType" {
                return edgetpu::DeviceType::kApexUsb;
            }),
        };

        let mut device_options = cpp!(unsafe [] -> EdgeTpuDeviceOptions as "edgetpu::EdgeTpuManager::DeviceOptions" {
            edgetpu::EdgeTpuManager::DeviceOptions x;
            x.reserve(4);
            return x;
        });

        for (key, val) in options {
            let ks = CString::new(key.as_str()).unwrap();
            let vs = CString::new(val.as_str()).unwrap();
            let k = ks.as_ptr();
            let v = vs.as_ptr();
            cpp!(unsafe [mut device_options as "edgetpu::EdgeTpuManager::DeviceOptions", k as "const char *", v as "const char *"] {
                device_options.insert({std::string(k), std::string(v)});
            });
        }

        let path_s = CString::new(path).unwrap();
        let path = path_s.as_ptr();

        let inner = cpp!(unsafe [device_type as "edgetpu::DeviceType", path as "const char *", mut device_options as "edgetpu::EdgeTpuManager::DeviceOptions"] -> InnerEdgeTpuContext as "std::shared_ptr<edgetpu::EdgeTpuContext>" {
            auto device_path = std::string(path);
            return get_edgetpu_manager_singleton()->OpenDevice(device_type, device_path, device_options);
        });

        let ok = cpp!(unsafe [inner as "std::shared_ptr<edgetpu::EdgeTpuContext>"] -> i32 as "int" {
            return !!inner;
        });

        match ok {
            0 => Err(EdgeTPUError::OpenFailed),
            _ => Ok(EdgeTpuContext {
                inner: Arc::new(Mutex::new(inner)),
            }),
        }
    }

    pub fn set_verbosity(&self, verbosity: i32) -> Result<(), EdgeTPUError> {
        let status = cpp!(unsafe [verbosity as "int"] -> i32 as "int" {
            return get_edgetpu_manager_singleton()->SetVerbosity(verbosity);
        });
        match status {
            0 => Err(EdgeTPUError::SetVerbosityFailed),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_devices() {
        let manager = EdgeTpuDeviceManager::get_singleton().expect("Failed to get singleton");
        let devices = manager.enumerate_devices();
        assert!(!devices.is_empty(), "No devices found");
    }
}