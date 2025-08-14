
/// Decision about how to execute a command
#[cfg(feature = "substrate_host_hook")]
#[derive(Debug, Clone)]
pub enum ExecDecision {
    /// Normal REPL submission
    Success(String),
    /// Host should run this command out-of-band (e.g., PTY)
    ExecuteHostCommand(String),
}

/// Trait for deciding whether a command should be executed by the host
#[cfg(feature = "substrate_host_hook")]
pub trait HostCommandDecider: Send + Sync {
    /// Decide how to handle the given command line
    fn decide(&self, line: &str) -> ExecDecision;
}