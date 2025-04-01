//! In-memory log buffer.

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A logged message.
pub struct LogMessage {
    pub metadata: &'static tracing::Metadata<'static>,
    pub timestamp: std::time::SystemTime,
    pub fields: Vec<(&'static str, serde_json::Value)>,
}

/// A slot in the log message buffer.
pub enum LogMessageSlot {
    /// An empty slot.
    Null,
    /// A message slot.
    Message(LogMessage),
}

/// A ring buffer that stores the last `N` log messages.
pub struct InMemoryLogBuffer {
    /// The buffer of log messages.
    buffer: Vec<Mutex<Vec<LogMessageSlot>>>,
    /// The index to write to.
    write_index: AtomicUsize,
    /// The oldest item in the wring.
    read_index: AtomicUsize,
    /// The capacity of the buffer.
    pub capacity: usize,
    /// The size of each slab.
    pub size_per_slab: usize,
}

impl InMemoryLogBuffer {
    /// Create a new in-memory log buffer.
    ///
    /// The buffer will store `size_per_slab * slabs` log messages.
    pub fn new(size_per_slab: usize, slabs: usize) -> std::sync::Arc<Self> {
        let mut buffer = Vec::with_capacity(slabs);
        buffer.resize_with(slabs, || {
            let mut slab = Vec::with_capacity(size_per_slab);
            slab.resize_with(size_per_slab, || LogMessageSlot::Null);
            Mutex::new(slab)
        });

        std::sync::Arc::new(Self {
            buffer,
            read_index: AtomicUsize::new(0),
            write_index: AtomicUsize::new(0),
            capacity: size_per_slab * slabs,
            size_per_slab,
        })
    }

    /// Get the slab index and slab offset for a logical index.
    pub fn get_slab_index(&self, index: usize) -> (usize, usize) {
        let logical_index = index % self.capacity;
        let slab_index = logical_index / self.size_per_slab;
        let slab_offset = logical_index % self.size_per_slab;

        (slab_index, slab_offset)
    }

    /// Append a log message to the buffer.
    pub fn append(&self, value: LogMessage) {
        let current_index: usize = self.write_index.fetch_add(1, Ordering::SeqCst);
        if current_index >= self.capacity {
            self.read_index.fetch_add(1, Ordering::Relaxed);
        }
        let (slab_index, slab_offset) = self.get_slab_index(current_index);

        let mut slab = self.buffer[slab_index].lock().unwrap();
        slab[slab_offset] = LogMessageSlot::Message(value);
    }

    /// Iterate over each log message.
    pub fn for_each(&self, mut f: impl FnMut(&LogMessage)) {
        let current_index = self.read_index.load(Ordering::SeqCst);
        let last_index = self.write_index.load(Ordering::SeqCst);

        let (start_slab_index, start_slab_offset) = self.get_slab_index(current_index);
        let (end_slab_index, end_slab_offset) = self.get_slab_index(last_index);
        for slab_index in start_slab_index..=end_slab_index {
            let slab = self.buffer[slab_index].lock().unwrap();
            let start_index = if slab_index == start_slab_index {
                start_slab_offset
            } else {
                0
            };
            let end_index = if slab_index == end_slab_index {
                end_slab_offset
            } else {
                self.size_per_slab
            };

            for slab_offset in start_index..end_index {
                if let LogMessageSlot::Message(value) = &slab[slab_offset] {
                    f(value);
                }
            }
        }
    }
}
