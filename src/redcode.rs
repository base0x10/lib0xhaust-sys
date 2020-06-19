use crate::internals;

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    DAT,
    SPL,
    MOV,
    DJN,
    ADD,
    JMZ,
    SUB,
    SEQ,
    SNE,
    SLT,
    JMN,
    JMP,
    NOP,
    MUL,
    MOD,
    DIV,
    LDP,
    STP,
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::DAT
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    DIRECT,
    IMMEDIATE,
    BINDIRECT,
    BPREDEC,
    BPOSTINC,
    AINDIRECT,
    APREDEC,
    APOSTINC,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::DIRECT
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Modifier {
    A,
    B,
    BA,
    AB,
    F,
    X,
    I,
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::F
    }
}

pub type Insn = internals::insn_t;

impl Default for Insn {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}

impl Insn {
    pub fn new(
        op: Opcode,
        modifier: Modifier,
        a_mode: Mode,
        a_field: u16,
        b_mode: Mode,
        b_field: u16,
    ) -> Insn {
        let op = match op {
            Opcode::DAT => internals::DAT,
            Opcode::SPL => internals::SPL,
            Opcode::MOV => internals::MOV,
            Opcode::DJN => internals::DJN,
            Opcode::ADD => internals::ADD,
            Opcode::JMZ => internals::JMZ,
            Opcode::SUB => internals::SUB,
            Opcode::SEQ => internals::SEQ,
            Opcode::SNE => internals::SNE,
            Opcode::SLT => internals::SLT,
            Opcode::JMN => internals::JMN,
            Opcode::JMP => internals::JMP,
            Opcode::NOP => internals::NOP,
            Opcode::MUL => internals::MUL,
            Opcode::MOD => internals::MODM,
            Opcode::DIV => internals::DIV,
            Opcode::LDP => internals::LDP,
            Opcode::STP => internals::STP,
        };

        let modifier = match modifier {
            Modifier::A => internals::mA,
            Modifier::B => internals::mB,
            Modifier::AB => internals::mAB,
            Modifier::BA => internals::mBA,
            Modifier::F => internals::mF,
            Modifier::X => internals::mX,
            Modifier::I => internals::mI,
        };

        let a_mode = match a_mode {
            Mode::DIRECT => internals::DIRECT,
            Mode::IMMEDIATE => internals::IMMEDIATE,
            Mode::BINDIRECT => internals::BINDIRECT,
            Mode::BPREDEC => internals::BPREDEC,
            Mode::BPOSTINC => internals::BPOSTINC,
            Mode::AINDIRECT => internals::AINDIRECT,
            Mode::APREDEC => internals::APREDEC,
            Mode::APOSTINC => internals::APOSTINC,
        };
        let b_mode = match b_mode {
            Mode::DIRECT => internals::DIRECT,
            Mode::IMMEDIATE => internals::IMMEDIATE,
            Mode::BINDIRECT => internals::BINDIRECT,
            Mode::BPREDEC => internals::BPREDEC,
            Mode::BPOSTINC => internals::BPOSTINC,
            Mode::AINDIRECT => internals::AINDIRECT,
            Mode::APREDEC => internals::APREDEC,
            Mode::APOSTINC => internals::APOSTINC,
        };

        // This is taken from insn.h The bit packing isn't
        // really useful in this day and age but we don't want to rewrite exhaust
        let op = (op << internals::moBITS) | modifier;
        let op = (op << internals::moPOS) | (b_mode << internals::mbPOS) | a_mode;
        return Insn {
            a: a_field,
            b: b_field,
            in_: op as u16,
        };
    }
}

impl std::fmt::Display for Insn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: reimplement dis1 from asm.c here
        write!(f, "{} {} {}", self.in_, self.a, self.b)
    }
}
