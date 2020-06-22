use crate::internals;
use crate::redcode;

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
    pub fn simulate(&mut self, warrior_positions: Vec<redcode::Field>) -> Vec<u32> {
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
            let dead_index = unsafe { *death_tab_ptr.offset(i as isize) };
            res.push(dead_index)
        }
        self.reset_round();
        return res;
    }

    fn reset_round(&mut self) {
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
