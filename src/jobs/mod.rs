use std::{fmt::Display, process::Child};

pub mod job_helper;

#[derive(PartialEq)]
pub enum JobStatus {
    Running = 1,
    Done = 2,
    Failed = 3
}

impl Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Running => f.pad("Running"),
            JobStatus::Done => f.pad("Done"),
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

    pub fn get_all(&self) -> Vec<&Job> {
        self.jobs.iter().collect()
    }

    pub fn update_statuses(&mut self) {
        for job in &mut self.jobs {
            match job.child.try_wait() {
                Ok(None) => job.status = JobStatus::Running,
                Ok(Some(status)) => {
                    if status.success() {
                        job.status = JobStatus::Done
                    } else {
                        job.status = JobStatus::Failed
                    }
                },
                Err(_) => job.status = JobStatus::Failed
            }
        }
    }

    pub fn get_done(&self) -> Vec<&Job> {
        self.jobs
            .iter()
            .filter(|j| j.status == JobStatus::Done)
            .collect()
    }

    pub fn clear_jobs(&mut self) {
        self.jobs.retain(|job| job.status == JobStatus::Running)
    }

    pub fn get_next_number(&self) -> u32 {
        let max_num = self.jobs.iter().map(|i| i.number).max().unwrap_or(0);
        max_num + 1
    }
}