use pulldown_cmark::*;
use std::{
    collections::HashMap,
    error::Error,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use goodmorning_bindings::services::v1::*;
use tokio::{
    fs,
    io::AsyncWriteExt,
    sync::oneshot::{self, Sender},
};
use log::*;

#[derive(Default)]
pub struct Jobs(pub Mutex<HashMap<i64, Arc<Mutex<Job>>>>);

#[derive(Default)]
pub struct Job {
    pub current: Vec<SingleJob>,
    pub queue: Vec<SingleJobWrapper>,
}

impl Jobs {
    pub async fn run_with_limit(
        &self,
        id: i64,
        task: SingleTask,
        max_concurrent: usize,
    ) -> Result<V1Response, Box<dyn Error>> {
        let arc = self
            .0
            .lock()
            .unwrap()
            .entry(id)
            .or_insert(Arc::default())
            .clone();

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
}

impl Job {
    async fn run_with_limit(
        arc: Arc<Mutex<Self>>,
        max_concurrent: usize,
        job: SingleJob,
    ) -> Result<V1Response, Box<dyn Error>> {
        let (tx, rx) = oneshot::channel();
        debug!("Queued job: {job:?}");
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
                debug!("Started job: {:?}", jobwrapper.job);
                jobwrapper.job.task.run(jobwrapper.tx, job.id).await;
                debug!("Finished job: {:?}", jobwrapper.job);
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

#[derive(Debug)]
#[derive(Clone)]
pub struct SingleJob {
    pub task: SingleTask,
    pub id: u64,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum SingleTask {
    Compile {
        from: FromFormat,
        to: ToFormat,
        source: PathBuf,
    },
}

impl SingleTask {
    async fn run(&self, tx: Sender<Result<V1Response, V1Error>>, taskid: u64) {
        let res = match self {
            Self::Compile { from, to, source } => {
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
                match (from, to) {
                    (FromFormat::Markdown, ToFormat::Html) => {
                        markdown_to_html(source, taskid).await
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

async fn markdown_to_html(source: &Path, taskid: u64) -> Result<V1Response, Box<dyn Error>> {
    if source.extension() != Some(OsStr::new("md")) {
        return Err(V1Error::ExtensionMismatch.into());
    }

    let md = fs::read_to_string(source).await?;
    let mut buf = String::new();
    html::push_html(&mut buf, Parser::new_ext(&md, Options::all()));

    let newfile = PathBuf::from(source).with_extension("html");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&newfile)
        .await?;
    file.write_all(buf.as_bytes()).await?;
    Ok(V1Response::Compiled {
        id: taskid,
        newpath: newfile.to_str().unwrap().to_string(),
        message: String::new(),
    })
}

pub enum JobError {}
