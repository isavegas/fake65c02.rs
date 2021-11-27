use fake65c02_sys::*;

use crate::MemoryLayout;

#[derive(Debug, Clone)]
pub struct State {
    context: *mut fake65c02_t,
    memory: MemoryLayout,
}

impl State {
    pub fn new() -> Self {
        Self {
            context: unsafe { new_fake65c02(std::ptr::null_mut()) },
            memory: Default::default(),
        }
    }
    pub fn reset(&mut self) {
        unsafe {
            reset65c02(self.context);
        }
    }
    pub fn step(&mut self) {
        unsafe {
            step65c02(self.context);
        }
    }
    pub fn exec(&mut self, tickcount: u32) {
        unsafe {
            exec65c02(self.context, tickcount);
        }
    }
    pub fn irq(&mut self) {
        unsafe {
            irq65c02(self.context);
        }
    }
    pub fn nmi(&mut self) {
        unsafe {
            nmi65c02(self.context);
        }
    }
}
