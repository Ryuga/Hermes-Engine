use crossbeam_channel::{bounded, Receiver, Sender};
use std::process::Command;
use std::path::PathBuf;
use std::io::Result;
use std::fs;

pub struct IsolateBox {
    pub id: i8,
    pub path: PathBuf,
}

pub struct BoxManager {
    box_pool: Receiver<IsolateBox>,
    sender: Sender<IsolateBox>,
}

impl IsolateBox {
    pub fn new(id: i8) -> IsolateBox {
        Self {
            id,
            path: PathBuf::from(format!("/var/lib/isolate/{}/box", id)),
        }
    }
    pub fn cleanup(&self) -> Result<()>{
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
        let _ = Command::new("pkill")
            .args(["-9", "-u", format!("isolate-{}", self.id.to_string()).as_str()])
            .output();
        Ok(())
    }
}


impl BoxManager {
    pub fn new(count: i8) -> Self {
        let (tx, rx) = bounded(count as usize);

        for i in 0..count{
            let status = Command::new("isolate")
                .args(["--box-id", &i.to_string(), "--init"]).status()
                .expect("Failed to execute command. Verify isolate installation.");

            if !status.success() {
                panic!("{}", format!("Failed to initialize box {}", i.to_string()));
            }

            let b = IsolateBox::new(i);
            tx.send(b).unwrap();
        }

        Self {
            box_pool: rx,
            sender: tx,
        }
    }

    pub fn acquire(&self) -> IsolateBox {
        self.box_pool.recv().expect("Box pool closed.")
    }

    pub fn release(&self, isolate_box: IsolateBox) {
        if let Err(e) = isolate_box.cleanup(){
            eprintln!("Cleanup failed for box: {} with: {}", isolate_box.id, e);
        }
        self.sender.send(isolate_box).unwrap();
    }
}
