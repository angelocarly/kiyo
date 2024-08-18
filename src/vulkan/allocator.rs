use std::sync::{Arc, Mutex, MutexGuard};
use gpu_allocator::vulkan::AllocatorCreateDesc;
use log::trace;
use crate::vulkan::LOG_TARGET;

pub struct AllocatorInner {
    pub allocator: gpu_allocator::vulkan::Allocator,
}

impl Drop for AllocatorInner {
    fn drop(&mut self) {
        trace!(target: LOG_TARGET, "Destroyed allocator");
    }
}

pub struct Allocator {
    pub(crate) inner: Arc<Mutex<AllocatorInner>>,
}

impl Allocator {
    pub fn new(desc: &AllocatorCreateDesc) -> Self {
        let allocator = Arc::new( Mutex::new(AllocatorInner { allocator: gpu_allocator::vulkan::Allocator::new(desc).expect("Failed to create allocator") } ) );

        trace!(target: LOG_TARGET, "Created allocator");

        Self {
            inner: allocator,
        }
    }

    pub fn handle(&self) -> MutexGuard<'_, AllocatorInner> {
        self.inner.lock().unwrap()
    }
}