use batchi::{job::JobExecutionPolicy, Batch, Batchi, JobConsumer, JobContext};

struct InsertBooksMessage {
    title: String,
}

struct BooksConsumer {
    batch: Batch<InsertBooksMessage>,
}

impl BooksConsumer {
    // todo: Can this be moved to the library without making JobConsumer generic??
    fn new(job_id: u32, job_name: &str, messages: Vec<InsertBooksMessage>) -> Box<dyn JobConsumer> {
        Box::new(BooksConsumer {
            batch: Batch {
                jobs: messages
                    .into_iter()
                    .map(|f| JobContext::new(job_id, job_name, f))
                    .collect(),
            },
        })
    }
}

impl JobConsumer for BooksConsumer {
    fn execute(&self) {
        for job in &self.batch.jobs {
            let job_name = job.name.to_string();
            println!("Job Name: {job_name}");
            println!("Book title: {}", job.data.title);
        }
    }
}

fn main() {
    let consumer = BooksConsumer::new(
        1,
        "Insert Books",
        vec![
            InsertBooksMessage {
                title: "The Way of Kings".to_owned(),
            },
            InsertBooksMessage {
                title: "Words of Radiance".to_owned(),
            },
            InsertBooksMessage {
                title: "Oathbringer".to_owned(),
            },
        ],
    );

    let batchi = Batchi::new()
        .consumer_with_policy(
            consumer,
            JobExecutionPolicy::new()
                .fetch_duration(2000)
                .jobs_per_batch(25),
        )
        .build();

    batchi.run();
}
