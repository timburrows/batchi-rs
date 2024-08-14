use crate::{job::JobExecutionPolicy, JobConsumer};

pub struct BatchiBuilder {
    job_consumers: Vec<(Box<dyn JobConsumer>, JobExecutionPolicy)>,
}

impl BatchiBuilder {
    pub fn build(self) -> Batchi {
        Batchi {
            job_consumers: self.job_consumers,
        }
    }

    pub fn consumer(mut self, consumer: Box<dyn JobConsumer>) {
        let policy = JobExecutionPolicy::new();
        self.job_consumers.push((consumer, policy));
    }

    pub fn consumer_with_policy(
        mut self,
        consumer: Box<dyn JobConsumer>,
        policy: JobExecutionPolicy,
    ) -> Self {
        self.job_consumers.push((consumer, policy));
        self
    }
}

pub struct Batchi {
    job_consumers: Vec<(Box<dyn JobConsumer>, JobExecutionPolicy)>,
}

impl Batchi {
    pub fn new() -> BatchiBuilder {
        BatchiBuilder {
            job_consumers: vec![],
        }
    }

    pub fn run(&self) {
        for (job, policy) in &self.job_consumers {
            job.execute();
        }
    }
}
