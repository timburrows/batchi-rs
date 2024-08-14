pub struct JobContext<T> {
    pub id: u32,
    pub name: String,
    pub data: T,
}

impl<T> JobContext<T> {
    pub fn new(id: u32, name: &str, data: T) -> Self {
        Self {
            id,
            name: name.to_owned(),
            data,
        }
    }
}

pub struct Batch<T> {
    pub jobs: Vec<JobContext<T>>,
}

/// Defines a unit of work to be executed by the batch orchestrator.
/// A JobContext<T> provides the message to be consumed and relevant job metadata.
pub trait JobConsumer {
    fn execute(&self);
}

pub struct JobExecutionPolicy {
    /// The length of time in milliseconds that the Worker will collect
    /// Jobs for each batch before execution
    fetch_duration: u32,

    /// The maximum number of Jobs that will be collected per batch
    /// before the batch is yielded to an JobExecutor<T> for execution
    jobs_per_batch: u32,
}

impl JobExecutionPolicy {
    pub fn new() -> JobExecutionPolicy {
        Self {
            fetch_duration: 1000,
            jobs_per_batch: 10,
        }
    }
    pub fn fetch_duration(mut self, v: u32) -> Self {
        self.fetch_duration = v;
        self
    }

    pub fn jobs_per_batch(mut self, v: u32) -> Self {
        self.jobs_per_batch = v;
        self
    }
}
