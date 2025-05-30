use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PerformanceMetric {
    pub phase: String,
    pub elapsed: f64,
    pub memory_usage: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PerformanceData {
    pub bytecode_size: usize,
    pub metrics: Vec<PerformanceMetric>,
    pub reused_programs: u64,
}

#[derive(serde::Serialize, Clone)]
pub struct FunctionEntryPoint {
    /// The original entry point function name.
    pub fn_name: String,
    /// The immediate instruction offset at which the entry function begins.
    pub imm: u64,
    /// The function selector (only `Some` for contract ABI methods).
    pub selector: Option<[u8; 4]>,
}

#[macro_export]
// Time the given expression and print/save the result.
macro_rules! time_expr {
    ($pkg_name:expr, $description:expr, $key:expr, $expression:expr, $build_config:expr, $data:expr) => {{
        use std::io::{BufRead, Read, Write};
        if let Some(cfg) = $build_config {
            if cfg.profile {
                println!("/dyno start {} {}", $pkg_name, $description);
                let output = { $expression };
                println!("/dyno stop {} {}", $pkg_name, $description);
                output
            } else if cfg.time_phases || cfg.metrics_outfile.is_some() {
                let expr_start = std::time::Instant::now();
                let output = { $expression };
                let elapsed = expr_start.elapsed();
                if cfg.time_phases {
                    println!("  Time elapsed to {}: {:?}", $description, elapsed);
                }
                if cfg.metrics_outfile.is_some() {
                    #[cfg(not(target_os = "macos"))]
                    let memory_usage = {
                        use sysinfo::{System, SystemExt};
                        let mut sys = System::new();
                        sys.refresh_system();
                        Some(sys.used_memory())
                    };
                    #[cfg(target_os = "macos")]
                    let memory_usage = None;

                    $data.metrics.push(PerformanceMetric {
                        phase: $key.to_string(),
                        elapsed: elapsed.as_secs_f64(),
                        memory_usage,
                    });
                }
                output
            } else {
                $expression
            }
        } else {
            $expression
        }
    }};
}
