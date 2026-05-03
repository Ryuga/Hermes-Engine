use crossbeam_channel::{bounded, Receiver, Sender};
use std::process::Command;
use std::path::PathBuf;
use std::io::Result;


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
            path: PathBuf::from(format!("/var/local/lib/isolate/{}/box", id)),
        }
    }
    pub fn cleanup(&self) -> Result<()>{
        // TODO
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
                panic!(format!("Failed to initialize box {}", i));
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
