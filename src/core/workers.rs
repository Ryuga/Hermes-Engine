use std::fmt::Debug;
use std::fs;
use std::io::Result;
use std::ops::Range;
use std::path::PathBuf;
use std::process::Command;

use crossbeam_channel::{bounded, Receiver, Sender};
use tracing::info;

pub trait Sandbox: Debug{
    fn id(&self) -> usize;
    fn path(&self) -> PathBuf;
    fn cleanup(&self) -> Result<()>;
}


#[derive(Debug)]
pub struct EphemeralBox {
    pub id: usize,
    pub path: PathBuf,
}

impl EphemeralBox {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            path: PathBuf::from(format!("/var/lib/isolate/{}/box", id)),
        }
    }
}

impl Sandbox for EphemeralBox {
    fn id(&self) -> usize {
        self.id
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn cleanup(&self) -> Result<()>{
        // Remove all temp files without unmounting.
        if self.path.exists() {
            for entry in fs::read_dir(&self.path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                } else {
                    fs::remove_file(path)?;
                }
            }
        }
        // Kill all process for that user.
        let target_user = format!("isolate-{}", self.id);
        let _ = Command::new("pkill")
            .args(["-9", "-u"])
            .arg(&target_user)
            .output();
        Ok(())
    }
}

#[derive(Debug)]
pub struct PersistentBox {
    pub id: usize,
    pub path: PathBuf,
}

impl PersistentBox {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            path: PathBuf::from(format!("/var/lib/isolate/{}/box", id)),
        }
    }
}

impl Sandbox for PersistentBox {
    fn id(&self) -> usize {
        self.id
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn cleanup(&self) -> Result<()>{
        Ok(())
    }
}

#[derive(Debug)]
pub struct BoxManager<T: Sandbox> {
    box_pool: Receiver<T>,
    sender: Sender<T>,
}



impl<T:Sandbox> BoxManager<T> {
    pub fn new<F> (id_range: Range<usize>, init_box: F) -> Self
    where
        F: Fn(usize) -> T,
    {

        let count = id_range.len();

        let (tx, rx) = bounded(count);

        for id in id_range {
            // Clean up previous boxes on restart.
            let _ = Command::new("isolate")
                .args(["--box-id", &id.to_string(), "--cg", "--cleanup"])
                .status();

            //TODO: Add warning for control group per kernel version
            let status = Command::new("isolate")
                .args(["--box-id", &id.to_string(), "--cg", "--init"]).status()
                .expect("Failed to execute command. Verify isolate installation.");

            if !status.success() {
                panic!("{}", format!("Failed to initialize box {}", id.to_string()));
            }

            let b = init_box(id);
            tx.send(b).unwrap();
        }

        info!("Workers in pool: {}", count);

        Self {
            box_pool: rx,
            sender: tx,
        }
    }

    pub fn acquire(&self) -> T {
        self.box_pool.recv().expect("Box pool closed.")
    }

    pub fn release(&self, target_box: T) {
        if let Err(e) = target_box.cleanup(){
            eprintln!("Cleanup failed for box: {} with: {}", target_box.id(), e);
        }
        self.sender.send(target_box).unwrap();
    }
}
