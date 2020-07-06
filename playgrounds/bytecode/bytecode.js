// currently at: When an instruction needs to receive parameters, it pops them off the stack like so:

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
        this.stack = []
        this.size = 0;
    }

    push(value) {
        this.stack.push(value);
        this.size += 1;
    }

    pop() {
        if(this.size > 0) {
            return this.stack[size]
        } else {
            throw new Error('stack does not have value');
        }
    }

  interpret(bytecode = []) {
    bytecode.forEach((bc) => {
      switch (instructionEnum[bc]) {
        case instructionEnum.setHealth: {
          setHealth(0, 100);
          break;
        }
        case instructionEnum.setWisdom {
            setWisdom(0, 100);
            break;
        }
      }
    });
  }
}
