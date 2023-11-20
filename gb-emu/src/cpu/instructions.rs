use std::sync::atomic::{
    AtomicU8,
    AtomicU16,
    Ordering::Relaxed,
  };
  
  use crate::{
    cpu::{
      Cpu,
      operand::{Reg16, Imm16, Imm8, Cond, IO8, IO16}
    },
    peripherals::Peripherals,
  };

  macro_rules! step {
    ($d:expr, {$($c: tt : $e:expr,)*}) => {
        static STEP: AtomicU8 = AtomicU8::new(0);
    }
  }

impl Cpu {
    pub fn nop(&mut self, bus: Peripherals) {
        self.fetch(bus);
    }
}