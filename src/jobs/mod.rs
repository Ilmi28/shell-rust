use std::{fmt::Display, process::Child};

pub enum JobStatus {
    Running = 1,
    Completed = 2,
    Failed = 3
}

impl Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Running => f.pad("Running"),
            JobStatus::Completed => f.pad("Completed"),
            JobStatus::Failed => f.pad("Failed")
        }
    }
}

pub struct Job {
    pub number: u32,
    pub cmd: String,
    pub child: Child,
    pub status: JobStatus
}

#[derive(Default)]
pub struct JobRegistry {
    jobs: Vec<Job>
}

impl JobRegistry {
    pub fn register(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn get_all(&self) -> &Vec<Job> {
        &self.jobs
    }

    pub fn update_statuses(&mut self) {
        for job in &mut self.jobs {
            match job.child.try_wait() {
                Ok(None) => job.status = JobStatus::Running,
                Ok(Some(status)) => {
                    if status.success() {
                        job.status = JobStatus::Completed
                    } else {
                        job.status = JobStatus::Failed
                    }
                },
                Err(_) => job.status = JobStatus::Failed
            }
        }
    }

    pub fn get_next_number(&self) -> u32 {
        let converted: u32 = match self.jobs.len().try_into() {
            Ok(n) => n,
            Err(_) => panic!("Value too large for u32.")
        };
        converted + 1
    }
}