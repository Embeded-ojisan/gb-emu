use crate::{
    cpu::Cpu,
    peripherals::Peripherals,
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