/// Exit status and captured process output.
#[derive(Debug)]
pub struct Output {
    /// Whether the child exited successfully.
    pub success: bool,
    /// Numeric process exit status, or None when no child ran or it was terminated by signal.
    pub exit_code: Option<i32>,
    /// Whether execution was skipped or the child was terminated because of the timeout.
    pub timed_out: bool,
    /// UTF-8 standard output, limited to one MiB.
    pub stdout: String,
    /// UTF-8 standard error, independently limited to one MiB.
    pub stderr: String,
}
