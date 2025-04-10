use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressTrack {
    pub(crate) pb: ProgressBar,
}

impl ProgressTrack {
    pub fn new(steps: u64, message: &str) -> Self {
        let pb = ProgressBar::new(steps);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"));
        pb.set_message(message.to_string());
        
        Self { pb }
    }
    
    pub fn increment(&self, message: Option<&str>) {
        self.pb.inc(1);
        if let Some(msg) = message {
            self.pb.set_message(msg.to_string());
        }
    }
    
    pub fn finish(&self, message: &str) {
        self.pb.finish_with_message(message.to_string());
    }
}