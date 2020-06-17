mod internals;
mod mars;
mod redcode;

pub mod lib0xhaust {
    pub use crate::mars::Simulator;
    pub use crate::redcode::Insn;
    pub use crate::redcode::Modifier;
    pub use crate::redcode::Mode;
    pub use crate::redcode::Opcode;
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
