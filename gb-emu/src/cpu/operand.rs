use crate::{
    cpu::Cpu,
    peripherals::Peripherals,
};

use std::sync::atomic::{
    AtomicU8,
    AtomicU16,
    Ordering::Relaxed,
};

#[derive(Clone, Copy, Debug)]
pub enum Reg8 { A, B, C, D, E, H, L }
#[derive(Clone, Copy, Debug)]
pub enum Reg16 { AF, BC, DE, HL, SP }
#[derive(Clone, Copy, Debug)]
pub struct Imm8;
#[derive(Clone, Copy, Debug)]
pub struct Imm16;
#[derive(Clone, Copy, Debug)]
pub enum Indirect { BC, DE, HL, CFF, HLD, HLI }
#[derive(Clone, Copy, Debug)]
pub enum Direct8 { D, DFF }
#[derive(Clone, Copy, Debug)]
pub struct Direct16;
#[derive(Clone, Copy, Debug)]
pub enum Cond { NZ, Z, NC, C }

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8) -> Option<()>;
}

pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16) -> Option<()>;
}

impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _:&Peripherals, src: Reg8) -> Option<u8> {
        Some(
            match src {
                Reg8::A => self.regs.a
            }
        )
    }

    fn write8(&mut self, _: &mut Peripherals, dst: Reg8, val: u8) -> Option<()> {
        Some(
            match dst {
                Reg8::A => self.regs.a = val,
            }
        )
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> Option<u16> {
        Some(
            match src {
                Reg16::AF => self.regs.af(),
                Reg16::SP => self.regs.sp = val,
            }
        )
    }
}

impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, _:Imm8) -> Option<u8> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);

        match STEP.load(Relaxed) {
            0 => {
                VAL8.store(bus.read(self.regs.pc), Relaxed);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                STEP.fetch_add(1, Relaxed);
                None
            },
            1 => {
                STEP.store(0, Relaxed);
                Some(VAL8.load(Relaxed))
            },
        }
    }

    fn write8(&mut self, _: Imm8, _:u8) -> Option<()> {
        unreachable!()
    }
}