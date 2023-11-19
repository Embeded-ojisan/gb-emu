
mod decode;
mod fetch;
mod instructions;
mod operand;
mod register;

#[derie(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}

pub struct Cpu {
    regs: Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            ctx: Ctx::default(),
        }
    }

    pub fn enumlate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }
}
