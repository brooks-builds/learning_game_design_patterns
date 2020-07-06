fn setHealth(wizardId: usize, amount: i32);
fn setWisdom(wizardId: usize, amount: i32);
fn setAgility(wizardId: usize, amount: i32);
fn playSound(soundId: usize);
fn spawnParticles(particleType: ParticleType);


enum ParticleType {}

enum Instruction {
    SetHealth,
    SetWisdom
}

struct VM {}

impl VM {
    pub fn interpret(&self, bytecodes: [Instruction]) {
        for bytecode in bytecodes {
            match bytecode {
                Instruction::SetHealth => setHealth(0, 100),
                Instruction::SetWisdom => setWisdom(0, 100),
            };
        }
    }
}