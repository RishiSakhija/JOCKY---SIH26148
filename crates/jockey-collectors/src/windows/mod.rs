//! Windows collectors module

pub mod proc_windows;
pub mod file_windows;
pub mod registry_windows;
pub mod logs_windows;
pub mod net_windows;

pub use proc_windows::ProcWindowsCollector;
pub use file_windows::FileWindowsCollector;
pub use registry_windows::RegistryWindowsCollector;
pub use logs_windows::LogsWindowsCollector;
pub use net_windows::NetWindowsCollector;