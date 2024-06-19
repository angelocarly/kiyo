use std::sync::{Arc, Mutex, MutexGuard};
use gpu_allocator::vulkan::AllocatorCreateDesc;

pub struct AllocatorInner {
    pub allocator: gpu_allocator::vulkan::Allocator,
}

impl Drop for AllocatorInner {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping allocator");
        }
    }
}

pub struct Allocator {
    pub(crate) inner: Arc<Mutex<AllocatorInner>>,
}

impl Allocator {
    pub fn new(desc: &AllocatorCreateDesc) -> Self {
        Self {
            inner: Arc::new( Mutex::new(AllocatorInner { allocator: gpu_allocator::vulkan::Allocator::new(desc).expect("Failed to create allocator") } ) ),
        }
    }

    pub fn handle(&self) -> MutexGuard<'_, AllocatorInner> {
        self.inner.lock().unwrap()
    }
}