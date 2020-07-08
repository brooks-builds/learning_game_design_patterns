fn setHealth(wizardId: usize, amount: i32);
fn setWisdom(wizardId: usize, amount: i32);
fn setAgility(wizardId: usize, amount: i32);
fn playSound(soundId: usize);
fn spawnParticles(particleType: ParticleType);


enum ParticleType {}

enum Instruction {
    SetHealth(i32, i32),
    SetWisdom(i32, i32),
}

enum CustomError {
    StackOverflow,
    StackUnderflow
}

struct VM {}

impl VM {
    pub fn interpret(&mut self, bytecodes: [Instruction]) -> Result<(), CustomError> {
        for bytecode in bytecodes {
            match bytecode {
                Instruction::SetHealth(wizardId, value) => setHealth(wizardId, value),
                Instruction::SetWisdom(wizardId, value) => setWisdom(wizardId, value),
            };
        }

        Ok(())
    }
}