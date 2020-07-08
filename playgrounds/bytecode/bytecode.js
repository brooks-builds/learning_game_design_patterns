function setHealth(wizardId, amount) {}
function setWisdom(wizardId, amount) {}
function setAgility(wizardId, amount) {}
function playSound(soundId) {}
function spawnParticles(particleType) {}

const instructionEnum = {
  setHealth: "setHealth",
  setWisdom: "setWisdom",
};

class VM {
  constructor() {
    this.stack = [];
    this.size = 0;
  }

  push(value) {
    this.stack.push(value);
    this.size += 1;
  }

  pop() {
    if (this.size > 0) {
      return this.stack[size];
    } else {
      throw new Error("stack does not have value");
    }
  }
  // stack : 
  interpret(bytecode = []) {
    for(let index = 0; index < this.size) {
      const instruction = bytecode[index];
      switch (instruction) {
        case instructionEnum.setHealth: {
          const amount = this.pop(); // 10
          const wizard = this.pop(); // 0
          setHealth(wizard, amount);
          break;
        }
        case instructionEnum.setWisdom: {
          const wizard = this.pop();
          const amount = this.pop();
          setWisdom(wizard, amount);
          break;
        }
        case INSTRUCTION_LITERAL: {
          index += 1;
          this.push(bytecode[index]);
          break;
        }
      }
    }
  }
}
