//! Executor (e.g. LSF, Slurm) API.

use std::any::Any;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;

use arcstr::ArcStr;
use derive_builder::Builder;

/// Job submission options.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecOpts {
    /// Number of CPUs to use.
    pub cpus: Option<usize>,
    /// Number of machines to use.
    pub machines: usize,
    /// Where to place logs.
    pub logs: LogOutput,
}

impl Default for ExecOpts {
    #[inline]
    fn default() -> Self {
        Self {
            cpus: None,
            machines: 1,
            logs: LogOutput::Stdio,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
/// Where to place logs generated by a job.
pub enum LogOutput {
    /// Save logs to standard output and standard error.
    #[default]
    Stdio,
    /// Save logs to a file.
    File(PathBuf),
}

/// A job executor.
pub trait Executor: Any + Send + Sync {
    /// Execute the given command with the given options, waiting until the command completes.
    fn execute(&self, command: Command, opts: ExecOpts) -> Result<(), crate::error::Error>;
}

/// Executes commands locally.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct LocalExecutor;

impl Executor for LocalExecutor {
    fn execute(&self, mut command: Command, opts: ExecOpts) -> Result<(), crate::error::Error> {
        if let LogOutput::File(ref path) = opts.logs {
            let fout = std::fs::File::create(path).map_err(Arc::new)?;
            let ferr = fout.try_clone().map_err(Arc::new)?;
            command.stdout(Stdio::from(fout)).stderr(Stdio::from(ferr));
        }

        let status = command.status().map_err(Arc::new)?;
        if !status.success() {
            return Err(crate::error::Error::CommandFailed(Arc::new(command)));
        }

        Ok(())
    }
}

/// An executor for submitting jobs to an LSF cluster.
#[derive(Clone, Debug, Eq, PartialEq, Builder)]
pub struct LsfExecutor {
    /// The command to use to submit jobs.
    #[builder(setter(into))]
    bsub: ArcStr,
    /// The queue to which jobs should be submitted.
    #[builder(setter(into, strip_option))]
    queue: Option<ArcStr>,
}

impl Default for LsfExecutor {
    fn default() -> Self {
        Self {
            bsub: arcstr::literal!("bsub"),
            queue: None,
        }
    }
}

impl LsfExecutor {
    /// A builder for constructing an [`LsfExecutor`].
    #[inline]
    pub fn builder() -> LsfExecutorBuilder {
        LsfExecutorBuilder::default()
    }

    /// Gets the LSF submission command.
    pub fn command(&self, command: &Command, opts: ExecOpts) -> Command {
        let mut submit = Command::new(&*self.bsub);

        // -K makes bsub wait until the job completes
        submit.arg("-K");
        if let Some(ref queue) = self.queue {
            submit.arg("-q").arg(queue.as_str());
        }
        if let Some(cpus) = opts.cpus {
            submit.arg("-n").arg(cpus.to_string());
        }
        submit.arg(command.get_program());
        for arg in command.get_args() {
            submit.arg(arg);
        }
        if let Some(dir) = command.get_current_dir() {
            submit.current_dir(dir);
        }

        for (key, val) in command.get_envs() {
            match val {
                None => submit.env_remove(key),
                Some(val) => submit.env(key, val),
            };
        }

        submit
    }
}

impl Executor for LsfExecutor {
    fn execute(&self, command: Command, opts: ExecOpts) -> Result<(), crate::error::Error> {
        let mut submit = self.command(&command, opts);

        let status = submit.status().map_err(Arc::new)?;
        if !status.success() {
            return Err(crate::error::Error::CommandFailed(Arc::new(submit)));
        }

        Ok(())
    }
}
