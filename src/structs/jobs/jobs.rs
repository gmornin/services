use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use goodmorning_bindings::services::v1::*;

use tokio::{
    fs,
    sync::oneshot::{self, Sender},
    time::timeout,
};

use super::{latex::pdflatex_latex2pdf, *};

#[derive(Default)]
pub struct Jobs(pub Mutex<HashMap<i64, Arc<Mutex<Job>>>>);

#[derive(Default)]
pub struct Job {
    pub current: Vec<SingleJob>,
    pub queue: Vec<SingleJobWrapper>,
}

impl Jobs {
    pub fn get(&self, id: i64) -> Arc<Mutex<Job>> {
        self.0
            .lock()
            .unwrap()
            .entry(id)
            .or_insert(Arc::default())
            .clone()
    }

    pub async fn run_with_limit(
        &self,
        id: i64,
        task: SingleTask,
        max_concurrent: usize,
    ) -> Result<V1Response, Box<dyn Error>> {
        let arc = self.get(id);

        Job::run_with_limit(
            arc,
            max_concurrent,
            SingleJob {
                task,
                id: fastrand::u64(..),
            },
        )
        .await
    }

    pub fn unqueue(&self, id: i64, jobid: u64) -> Result<(), V1Error> {
        self.0
            .lock()
            .unwrap()
            .entry(id)
            .or_insert(Arc::default())
            .lock()
            .unwrap()
            .unqueue(jobid)
    }
}

impl Job {
    pub fn unqueue(&mut self, jobid: u64) -> Result<(), V1Error> {
        match self.queue.iter().position(|job| job.job.id == jobid) {
            Some(i) => self.queue.remove(i),
            None => return Err(V1Error::JobNotFound),
        };

        Ok(())
    }

    async fn run_with_limit(
        arc: Arc<Mutex<Self>>,
        max_concurrent: usize,
        job: SingleJob,
    ) -> Result<V1Response, Box<dyn Error>> {
        let (tx, rx) = oneshot::channel();
        let job = SingleJobWrapper { job, tx };

        {
            let mut unlocked = arc.lock().unwrap();
            unlocked.queue.push(job);
            unlocked.bump(max_concurrent, &arc);
        }

        Ok(rx.await??)
    }

    fn bump(&mut self, max_concurrent: usize, arc: &Arc<Mutex<Self>>) {
        while self.current.len() < max_concurrent && !self.queue.is_empty() {
            let jobwrapper = self.queue.remove(0);
            let job = jobwrapper.job.clone();

            let arc = arc.clone();

            let _handle = tokio::task::spawn(async move {
                timeout(
                    Duration::from_secs(120),
                    jobwrapper.job.task.run(jobwrapper.tx, job.id),
                )
                .await
                .unwrap();
                let mut unlocked = arc.lock().unwrap();
                unlocked.done(jobwrapper.job.id).unwrap();
                unlocked.bump(max_concurrent, &arc)
            });
            self.current.push(job);
        }
    }

    fn done(&mut self, id: u64) -> Result<(), V1Error> {
        for (i, job) in self.current.iter().enumerate() {
            if job.id == id {
                self.current.remove(i);
                return Ok(());
            }
        }

        Err(V1Error::JobNotFound)
    }
}

pub struct SingleJobWrapper {
    pub job: SingleJob,
    pub tx: Sender<Result<V1Response, V1Error>>,
}

#[derive(Debug, Clone)]
pub struct SingleJob {
    pub task: SingleTask,
    pub id: u64,
}

#[derive(Debug, Clone)]
pub enum SingleTask {
    Compile {
        from: FromFormat,
        compiler: Compiler,
        to: ToFormat,
        source: PathBuf,
        user_path: PathBuf,
        restrict_path: PathBuf,
    },
}

impl SingleJob {
    pub fn to_v1(&self) -> V1Job {
        V1Job {
            id: self.id,
            task: self.task.to_v1(),
        }
    }
}

impl SingleTask {
    pub fn to_v1(&self) -> V1Task {
        match self {
            Self::Compile {
                from,
                compiler,
                to,
                user_path,
                ..
            } => V1Task::Compile {
                from: *from,
                to: *to,
                compiler: *compiler,
                path: user_path.to_str().unwrap().to_string(),
            },
        }
    }

    async fn run(&self, tx: Sender<Result<V1Response, V1Error>>, taskid: u64) {
        let res = match self {
            Self::Compile {
                from,
                to,
                compiler,
                source,
                user_path,
                restrict_path,
            } => {
                if !match fs::try_exists(&source).await {
                    Ok(b) => b,
                    Err(e) => {
                        tx.send(Err(V1Error::External {
                            content: e.to_string(),
                        }))
                        .unwrap();
                        return;
                    }
                } {
                    tx.send(Err(V1Error::FileNotFound)).unwrap();
                    return;
                }
                match (from, to, compiler) {
                    (
                        FromFormat::Markdown,
                        ToFormat::Html,
                        Compiler::Default | Compiler::PulldownCmark,
                    ) => pulldown_cmark_md2html(source, taskid, user_path).await,
                    (FromFormat::Latex, ToFormat::Pdf, Compiler::Default | Compiler::Pdflatex) => {
                        pdflatex_latex2pdf(source, taskid, user_path, restrict_path).await
                    }
                    _ => {
                        tx.send(Err(V1Error::InvalidCompileRequest)).unwrap();
                        return;
                    }
                }
            }
        };

        let res = match res {
            Ok(res) => Ok(res),
            Err(e) => Err(match e.downcast::<V1Error>() {
                Ok(e) => *e,
                Err(e) => V1Error::External {
                    content: e.to_string(),
                },
            }),
        };

        tx.send(res).unwrap();
    }
}
