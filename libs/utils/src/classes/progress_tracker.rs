use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

/// A struct to track progress and estimate time remaining for a process.
///
/// The `ProgressTracker` struct maintains a history of progress updates and uses this
/// information to calculate an estimated time of arrival (ETA) for completion.
///
/// # Fields
/// - `last_cycle_progress`: The progress value from the last update cycle.
/// - `eta_timer`: A timer to measure elapsed time for progress calculation.
/// - `progress_history`: A history of progress updates with their timestamps.
/// - `current_eta`: The current estimated time of arrival (ETA) for completion.
/// - `last_eta_update`: The last time the ETA was updated.
/// - `eta_history`: A history of calculated ETAs for smoothing.
/// - `eta_update_timer`: A timer to control the frequency of ETA updates.
///
pub struct ProgressTracker {
    last_cycle_progress: f32,

    progress_history: VecDeque<(SystemTime, f32)>,
    eta_history: VecDeque<f32>,

    last_eta_update: SystemTime,
    eta_update_timer: SystemTime,

    eta_timer: SystemTime,
    current_eta: f32,
}

impl ProgressTracker {
    /// Creates a new `ProgressTracker` instance.
    ///
    /// Initializes the struct with default values and sets the initial time points.
    ///
    /// # Returns
    /// A new `ProgressTracker` instance.
    ///
    /// # Examples
    /// ```
    /// let tracker = ProgressTracker::new();
    /// ```
    pub fn new() -> Self {
        ProgressTracker {
            last_cycle_progress: 0.0,
            eta_timer: SystemTime::now(),
            progress_history: VecDeque::new(),
            current_eta: 0.0,
            last_eta_update: SystemTime::now(),
            eta_history: VecDeque::new(),
            eta_update_timer: SystemTime::now(),
        }
    }

    /// Updates the progress tracker with the current progress of the task.
    ///
    /// This method should be called periodically to update the progress and
    /// calculate the ETA based on the progress history.
    ///
    /// # Arguments
    /// * `curr` - The current progress value (e.g., completed units of work).
    /// * `total` - The total value for completion (e.g., total units of work).
    ///
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if the update
    ///   is successful, otherwise returns an error.
    ///
    /// # Examples
    /// ```
    /// let mut tracker = ProgressTracker::new();
    /// tracker.update(50.0, 100.0).unwrap();
    /// ```
    pub fn update(&mut self, curr: f32, total: f32) -> Result<(), Box<dyn std::error::Error>> {
        let current_progress = curr / total;

        if self.last_cycle_progress < 0.01 {
            self.eta_timer = SystemTime::now();
            self.last_cycle_progress = current_progress;
        }

        let now = SystemTime::now();
        self.progress_history.push_back((now, current_progress));

        while let Some((time, _)) = self.progress_history.front() {
            if now.duration_since(*time)? > Duration::from_secs(5) {
                self.progress_history.pop_front();
            } else {
                break;
            }
        }

        if self.progress_history.len() > 1 {
            let (start_time, start_progress) = self.progress_history.front().unwrap();
            let elapsed = now.duration_since(*start_time)?.as_secs_f32();
            let progress_diff = current_progress - start_progress;
            let progress_remaining = 1.0 - current_progress;
            if progress_diff > 0.0 {
                let eta = (elapsed / progress_diff) * progress_remaining;
                self.eta_history.push_back(eta);
                if self.eta_history.len() > 5 {
                    self.eta_history.pop_front();
                }
                self.last_eta_update = now;
            }
        }

        if now.duration_since(self.eta_update_timer)? > Duration::from_secs(3) && !self.eta_history.is_empty() {
            self.current_eta = self.eta_history.iter().sum::<f32>() / self.eta_history.len() as f32;
            self.eta_update_timer = now;
        }

        self.last_cycle_progress = current_progress;

        Ok(())
    }

    /// Gets the current estimated time of arrival (ETA) for completion.
    ///
    /// This value is updated periodically based on the progress history.
    ///
    /// # Returns
    /// * `f32` - The current ETA in seconds.
    ///
    /// # Examples
    /// ```
    /// let tracker = ProgressTracker::new();
    /// tracker.update(50.0, 100.0).unwrap();
    /// let eta = tracker.get_eta();
    /// ```
    pub fn get_eta(&self) -> f32 {
        self.current_eta
    }
}
