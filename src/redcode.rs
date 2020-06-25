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
pub type Field = internals::field_t;

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
        a_field: Field,
        b_mode: Mode,
        b_field: Field,
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
        let op_bits = (self.in_ as u32 >> internals::opPOS) & internals::opMASK;
        let modifier_bits = (self.in_ as u32 >> internals::moPOS) & internals::moMASK;
        let a_mode_bits = (self.in_ as u32 >> internals::maPOS) & internals::mMASK;
        let b_mode_bits = (self.in_ as u32 >> internals::mbPOS) & internals::mMASK;

        let op = match op_bits {
            internals::DAT => "dat",
            internals::SPL => "spl",
            internals::MOV => "mov",
            internals::DJN => "djn",
            internals::ADD => "add",
            internals::JMZ => "jmz",
            internals::SUB => "sub",
            internals::SEQ => "seq",
            internals::SNE => "sne",
            internals::SLT => "slt",
            internals::JMN => "jmn",
            internals::JMP => "jmp",
            internals::NOP => "nop",
            internals::MUL => "mul",
            internals::MODM => "mod",
            internals::DIV => "div",
            internals::LDP => "ldp",
            internals::STP => "STP",
            _ => panic!(
                "disassembled a poorly formatted insn, {}, {}, {}",
                self.in_, self.a, self.b
            ),
        };

        let modifier = match modifier_bits {
            internals::mA => "a",
            internals::mB => "b",
            internals::mAB => "ab",
            internals::mBA => "ab",
            internals::mX => "x",
            internals::mF => "f",
            internals::mI => "i",
            _ => panic!(
                "disassembled a poorly formed insn, {}, {}, {}",
                self.in_, self.a, self.b
            ),
        };
        let a_mode = match a_mode_bits {
            internals::DIRECT => "$",
            internals::IMMEDIATE => "#",
            internals::BINDIRECT => "@",
            internals::BPREDEC => "<",
            internals::BPOSTINC => ">",
            internals::AINDIRECT => "*",
            internals::APREDEC => "{",
            internals::APOSTINC => "}",
            _ => panic!(
                "disassembled a poorly formed insn, {}, {}, {}",
                self.in_, self.a, self.b
            ),
        };
        let b_mode = match b_mode_bits {
            internals::DIRECT => "$",
            internals::IMMEDIATE => "#",
            internals::BINDIRECT => "@",
            internals::BPREDEC => "<",
            internals::BPOSTINC => ">",
            internals::AINDIRECT => "*",
            internals::APREDEC => "{",
            internals::APOSTINC => "}",
            _ => panic!(
                "disassembled a poorly formed insn, {}, {}, {}",
                self.in_, self.a, self.b
            ),
        };
        write!(
            f,
            "{}.{} {}{}, {}{}",
            op, modifier, a_mode, self.a, b_mode, self.b
        )
    }
}

pub fn assemble_warrior(fname: &str, coresize: u32) -> Result<(u32, Vec<Insn>), &'static str> {
    let fname = std::ffi::CString::new(fname).expect("Could not convert string to CString");
    let bytes = fname.into_bytes_with_nul();
    let ptr: *const std::os::raw::c_char = bytes.as_ptr() as *const i8;

    let mut w = internals::warrior_t {
        code: [Default::default(); 100],
        len: 0,
        start: 0,
        have_pin: 0,
        pin: 0,
        name: std::ptr::null_mut(),
        no: 0,
    };
    unsafe {
        internals::asm_fname(ptr, &mut w, coresize);
    };

    return Ok((w.start, w.code.to_vec()));
}
