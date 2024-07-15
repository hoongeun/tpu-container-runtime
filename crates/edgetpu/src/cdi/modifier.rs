use cdi::{CDIDevice, Spec as CDISpec, SpecBuilder as CDISpecBuilder, CDI};
use log::{debug, warn};
use oci_spec::runtime::{Spec, SpecBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    nvidia_container_runtime_config: NvidiaContainerRuntimeConfig,
    accept_device_list_as_volume_mounts: bool,
    accept_envvar_unprivileged: bool,
    nvidia_ctk_config: NvidiaCTKConfig,
    nvidia_container_cli_config: NvidiaContainerCLIConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct NvidiaContainerRuntimeConfig {
    modes: Modes,
}

#[derive(Serialize, Deserialize, Debug)]
struct Modes {
    cdi: CDIMode,
}

#[derive(Serialize, Deserialize, Debug)]
struct CDIMode {
    annotation_prefixes: Vec<String>,
    spec_dirs: Vec<String>,
    default_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NvidiaCTKConfig {
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NvidiaContainerCLIConfig {
    root: String,
}

fn new_cdi_modifier(
    cfg: &Config,
    oci_spec: &Spec,
) -> Result<Option<Spec>, Box<dyn Error>> {
    let devices = get_devices_from_spec(oci_spec, cfg)?;
    if devices.is_empty() {
        debug!(logger, "No devices requested; no modification required.");
        return Ok(None);
    }
    debug!(logger, "Creating CDI modifier for devices: {:?}", devices);

    let automatic_devices = filter_automatic_devices(&devices);
    if automatic_devices.len() != devices.len() && !automatic_devices.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Requesting a CDI device with vendor 'runtime.nvidia.com' is not supported when requesting other CDI devices",
        )));
    }
    if !automatic_devices.is_empty() {
        match new_automatic_cdi_spec_modifier(cfg, &automatic_devices) {
            Ok(modifier) => return Ok(Some(modifier)),
            Err(e) => {
                warn!(
                    "Failed to create the automatic CDI modifier: {:?}", e
                );
                debug!(logger, "Falling back to the standard CDI modifier");
            }
        }
    }

    CDI::new(&cdi::Config {
        logger: Some(logger.clone()),
        devices: Some(devices),
        spec_dirs: Some(
            cfg.nvidia_container_runtime_config
                .modes
                .cdi
                .spec_dirs
                .clone(),
        ),
        ..Default::default()
    })
    .map_err(|e| Box::new(e) as Box<dyn Error>)
}

fn get_devices_from_spec(oci_spec: &Spec, cfg: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let raw_spec = oci_spec.to_owned();

    let annotation_devices = get_annotation_devices(
        &cfg.nvidia_container_runtime_config
            .modes
            .cdi
            .annotation_prefixes,
        &raw_spec.annotations,
    )?;
    if !annotation_devices.is_empty() {
        return Ok(annotation_devices);
    }

    // Example structure for Container representation
    #[derive(Serialize, Deserialize, Debug)]
    struct Container {
        cdi_devices_from_mounts: Vec<String>,
        devices_from_envvars: Vec<String>,
    }

    let container: Container = serde_json::from_value(raw_spec)?;
    if cfg.accept_device_list_as_volume_mounts {
        let mount_devices = container.cdi_devices_from_mounts;
        if !mount_devices.is_empty() {
            return Ok(mount_devices);
        }
    }

    let env_devices = container.devices_from_envvars;
    let mut devices = Vec::new();
    let mut seen = HashMap::new();
    for name in env_devices {
        let name = if !is_qualified_name(&name) {
            format!(
                "{}={}",
                cfg.nvidia_container_runtime_config.modes.cdi.default_kind, name
            )
        } else {
            name
        };
        if seen.get(&name).is_some() {
            debug!(logger, "Ignoring duplicate device {:?}", name);
            continue;
        }
        devices.push(name.clone());
        seen.insert(name, true);
    }

    if devices.is_empty() {
        return Ok(vec![]);
    }

    if cfg.accept_envvar_unprivileged || is_privileged(&raw_spec) {
        return Ok(devices);
    }

    warn!(
        logger,
        "Ignoring devices specified in NVIDIA_VISIBLE_DEVICES: {:?}", devices
    );

    Ok(vec![])
}

fn get_annotation_devices(
    prefixes: &[String],
    annotations: &HashMap<String, String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut devices_by_key = HashMap::new();
    for (key, value) in annotations {
        for prefix in prefixes {
            if key.starts_with(prefix) {
                devices_by_key.insert(
                    key.clone(),
                    value.split(',').map(String::from).collect::<Vec<_>>(),
                );
            }
        }
    }

    let mut seen = HashMap::new();
    let mut annotation_devices = Vec::new();
    for (key, devices) in devices_by_key {
        for device in devices {
            if !is_qualified_name(&device) {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("invalid device name {:?} in annotation {:?}", device, key),
                )));
            }
            if seen.get(&device).is_some() {
                continue;
            }
            annotation_devices.push(device.clone());
            seen.insert(device, true);
        }
    }

    Ok(annotation_devices)
}

fn filter_automatic_devices(devices: &[String]) -> Vec<String> {
    let mut automatic = Vec::new();
    for device in devices {
        let (vendor, class, _) = parse_device(device);
        if vendor == "runtime.nvidia.com" && class == "gpu" {
            automatic.push(device.clone());
        }
    }
    automatic
}

fn new_automatic_cdi_spec_modifier(
    cfg: &Config,
    devices: &[String],
) -> Result<Spec, Box<dyn Error>> {
    debug!(
        logger,
        "Generating in-memory CDI specs for devices {:?}", devices
    );
    let spec = generate_automatic_cdi_spec(logger, cfg, devices)?;
    CDI::new(&cdi::Config {
        logger: Some(logger.clone()),
        spec: Some(spec),
        ..Default::default()
    })
    .map_err(|e| Box::new(e) as Box<dyn Error>)
}

fn generate_automatic_cdi_spec(
    cfg: &Config,
    devices: &[String],
) -> Result<CDISpec, Box<dyn Error>> {
    let cdilib = cdi::CDILibrary::new(&cdi::Config {
        logger: Some(logger.clone()),
        nvidia_cdi_hook_path: Some(cfg.nvidia_ctk_config.path.clone()),
        driver_root: Some(cfg.nvidia_container_cli_config.root.clone()),
        vendor: Some("runtime.nvidia.com".to_string()),
        class: Some("gpu".to_string()),
        ..Default::default()
    })?;

    let identifiers: Vec<_> = devices
        .iter()
        .map(|device| {
            let (_, _, id) = parse_device(device);
            id
        })
        .collect();

    let device_specs = cdilib.get_device_specs_by_id(&identifiers)?;
    let common_edits = cdilib.get_common_edits()?;

    Ok(CDISpecBuilder::default()
        .with_device_specs(device_specs)
        .with_edits(common_edits.container_edits)
        .with_vendor("runtime.nvidia.com".to_string())
        .with_class("gpu".to_string())
        .build()?)
}

fn is_qualified_name(name: &str) -> bool {
    // Implement your logic to check if a name is qualified
    true
}

fn is_privileged(spec: &Spec) -> bool {
    // Implement your logic to check if a spec is privileged
    true
}

fn parse_device(device: &str) -> (String, String, String) {
    // Implement your logic to parse a device string
    (String::new(), String::new(), String::new())
}
