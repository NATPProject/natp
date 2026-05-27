//! Owns a direct child and attempts to kill and reap it on drop.

use std::{
    io,
    process::{Child, Command, ExitStatus},
};

pub(super) struct RunningChild(Child);

impl RunningChild {
    pub(super) fn spawn(command: &mut Command) -> io::Result<Self> {
        command.spawn().map(Self)
    }

    pub(super) fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        self.0.try_wait()
    }

    pub(super) fn terminate(&mut self) -> io::Result<ExitStatus> {
        self.0.kill()?;

        self.0.wait()
    }
}

impl Drop for RunningChild {
    fn drop(&mut self) {
        // Drop cannot propagate errors; termination reports kill and wait failures.
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
