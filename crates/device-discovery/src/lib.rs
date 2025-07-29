use std::process::Stdio;

/// The capabilities of an accelerator.
#[derive(Debug, Clone)]
pub enum AcceleratorCapability {
    Cuda {
        /// Compute capability version (e.g., "8.9")
        compute_capability: String,
    },
}

/// Information about an accelerator.
#[derive(Debug, Clone)]
pub struct AcceleratorInfo {
    /// The name of the device
    pub name: String,
    /// The compute capability of the device
    pub capabilities: AcceleratorCapability,
    /// Total memory in bytes
    pub memory_total_bytes: i64,
    /// An index associated with the device
    pub index: Option<i64>,
}

impl AcceleratorInfo {
    /// Whether the device is a CUDA-capable GPU.
    pub fn is_cuda(&self) -> bool {
        matches!(self.capabilities, AcceleratorCapability::Cuda { .. })
    }
}

/// Information about a discovered GPU
#[derive(Debug, Clone)]
struct NvidiaGpuInfo {
    /// The product name of the GPU (e.g., "NVIDIA GeForce RTX 4090")
    pub name: String,
    /// Total GPU memory in megabytes
    pub memory_total_mb: i64,
    /// Compute capability version (e.g., "8.9")
    pub compute_capability: String,
    /// Zero-based index of the GPU in the system
    pub index: i32,
}

impl From<NvidiaGpuInfo> for AcceleratorInfo {
    fn from(gpu: NvidiaGpuInfo) -> Self {
        AcceleratorInfo {
            name: gpu.name,
            memory_total_bytes: gpu.memory_total_mb * 1024 * 1024, // Convert MB to bytes
            capabilities: AcceleratorCapability::Cuda {
                compute_capability: gpu.compute_capability,
            },
            index: Some(gpu.index as i64),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeviceDiscoveryError {
    #[error("Failed to execute nvidia-smi: {0}")]
    CommandFailed(#[from] std::io::Error),
    #[error("nvidia-smi command failed with exit code {code}: {stderr}")]
    CommandExitError { code: i32, stderr: String },
    #[error("Failed to parse nvidia-smi output: {0}")]
    ParseError(String),
}

/// Discovers NVIDIA GPUs using nvidia-smi and returns their information
fn discover_nvidia_gpus() -> Result<Vec<NvidiaGpuInfo>, DeviceDiscoveryError> {
    let output = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.total,compute_cap",
            "--format=csv,noheader,nounits",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let code = output.status.code().unwrap_or(-1);
        return Err(DeviceDiscoveryError::CommandExitError {
            code,
            stderr: stderr.to_string(),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line_count = stdout.lines().size_hint().0;
    let mut gpus = Vec::with_capacity(line_count);
    for (index, line) in stdout.lines().enumerate() {
        let parts: Vec<&str> = line.split(", ").collect();
        if parts.len() != 3 {
            continue; // Skip malformed lines
        }

        let name = parts[0].to_string();
        let memory_total = parts[1].parse::<u64>().unwrap_or(0);
        let compute_capability = parts[2].to_string();

        gpus.push(NvidiaGpuInfo {
            name,
            memory_total_mb: memory_total as i64,
            compute_capability,
            index: index as i32,
        });
    }

    Ok(gpus)
}

pub fn discover_devices() -> Result<Vec<AcceleratorInfo>, DeviceDiscoveryError> {
    let mut devices: Vec<AcceleratorInfo> = Vec::new();
    if cfg!(not(target_os = "macos")) {
        let nvidia_gpus = discover_nvidia_gpus()?;
        devices.reserve(nvidia_gpus.len());
        devices.extend(nvidia_gpus.into_iter().map(AcceleratorInfo::from));
    }
    Ok(devices)
}

/// Check if there is a CUDA-capable GPU available on the system.
pub fn has_cuda_device() -> Result<bool, DeviceDiscoveryError> {
    let devices = discover_devices()?;
    Ok(devices.iter().any(|d| d.is_cuda()))
}
