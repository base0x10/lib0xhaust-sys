mod internals {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod redcode {
    use crate::internals;

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

    pub enum Modifier {
        A,
        B,
        BA,
        AB,
        F,
        X,
        I,
    }

    pub type Insn = internals::insn_t;

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
}

pub mod mars {
    use crate::internals;

    pub struct Simulator<'a> {
        pub sim: &'a mut internals::SimState_t,
    }

    impl<'a> Simulator<'a> {
        pub fn new(nwar: u32, coresize: u32, processes: u32, cycles: u32, pspace: u32) -> Self {
            let res = unsafe { internals::sim_alloc(nwar, coresize, processes, cycles, pspace) };
            if res.is_null() {
                panic!("Failed to allocated space for a new simulator within 0xhaust c library")
            }
            let sim = unsafe { &mut *res };

            debug_assert!(sim.numWarriors == nwar);
            debug_assert!(sim.coreSize == coresize);
            debug_assert!(sim.maxProcesses == processes);
            debug_assert!(sim.pspaceSize == pspace);
            debug_assert!(sim.cycles == cycles);

            return Simulator { sim: sim };
        }

        /// Runs a simulation to completion
        ///
        /// Returns a vector of dead warriors ordered from first death to last death
        /// Warrior indices not in the result survived until the end of the simulation
        pub fn simulate(&mut self, warrior_positions: Vec<u16>) -> Vec<u16> {
            let mut death_tab = Vec::with_capacity(self.sim.numWarriors as usize);
            for _ in 0..self.sim.numWarriors {
                death_tab.push(0);
            }

            let war_pos_ptr = warrior_positions.as_ptr();
            let death_tab_ptr = death_tab.as_mut_ptr();

            let num_alive: u32 =
                unsafe { internals::sim_simulate(self.sim, war_pos_ptr, death_tab_ptr) } as u32;

            let mut res = Vec::with_capacity((self.sim.numWarriors - num_alive) as usize);
            for i in 0..self.sim.numWarriors - num_alive {
                let dead_index = unsafe { *death_tab_ptr.offset(i as isize) as u16 };
                res.push(dead_index)
            }
            return res;
        }

        pub fn reset_round(&mut self) {
            unsafe { internals::sim_reset_round(self.sim) };
        }

        pub fn reset_battle(&mut self) {
            unsafe { internals::sim_reset_battle(self.sim) };
        }

        pub fn load_warrior(&mut self, pos: u32, code: &[crate::redcode::Insn]) {
            // TODO: on error return?
            unsafe {
                internals::sim_load_warrior(self.sim, pos, code.as_ptr(), code.len() as u32);
            }
        }
    }

    impl<'a> Drop for Simulator<'a> {
        fn drop(self: &mut Self) {
            unsafe { internals::sim_free(self.sim) }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn allocate_sim() {
        let s: crate::mars::Simulator =
            crate::mars::Simulator::new(2, 8_000, 8_000, 80_000, 8_000 / 16);
        drop(s);
    }

    fn is_clear(sim: &crate::mars::Simulator) -> bool {
        for i in 0..sim.sim.coreSize {
            let insn: crate::redcode::Insn = unsafe { *sim.sim.coreMem.offset(i as isize) };
            let cleared = crate::redcode::Insn { a: 0, b: 0, in_: 0 };
            if !(cleared.a == insn.a && cleared.b == insn.b && cleared.in_ == insn.in_) {
                return false;
            };
        }
        return true;
    }

    #[test]
    fn test_reset_clears_core() {
        let mut s = crate::mars::Simulator::new(2, 8_000, 8_000, 80_000, 8_000 / 16);
        assert!(is_clear(&s));

        let war: Vec<crate::redcode::Insn> = vec![crate::redcode::Insn {
            a: 12,
            b: 34,
            in_: 123,
        }];

        s.load_warrior(0, &war);
        assert!(!is_clear(&s));
        s.reset_round();
        assert!(is_clear(&s));

        s.load_warrior(0, &war);
        assert!(!is_clear(&s));
        s.reset_battle();
        assert!(is_clear(&s));
    }
}
