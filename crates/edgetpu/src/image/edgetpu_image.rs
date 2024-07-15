use oci_spec::mount::Mount;

pub struct EdgeTPU {
    env: Map<str, str>,
    mounts: Vec<Mount>,
}

