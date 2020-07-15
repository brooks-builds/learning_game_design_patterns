const SOUND_BLOOP = 1;
const VOLUME_MAX = 1;

function findOpenChannel() {
  return 1;
}

function startSound(resource, channel, volume) {}

function loadSound(id) {}

function createPlayMessage(id, volume) {
  return {
    id,
    volume,
  };
}

class Audio {
  constructor() {
    this.MAX_PENDING = 16;
    this.pending = new Array(this.MAX_PENDING);
    this.init();
  }
  init() {
    this.head = 0;
    this.tail = 0;
  }
  playSound(id, volume) {
    if ((this.tail + 1) % this.MAX_PENDING == this.head)
      return new Error("Audio queue overflow");
    for (
      let index = this.head;
      index != this.tail;
      index = (index + 1) % this.MAX_PENDING
    ) {
      if (this.pending[index].id == id) {
        this.pending[index].volume = Math.max(
          volume,
          this.pending[index].volume
        );
        return;
      }
    }
    this.pending[this.tail] = createPlayMessage(id, volume);
    this.tail = (this.tail + 1) % this.MAX_PENDING;
  }
  update() {
    if (this.head == this.tail) return;

    const resource = loadSound(this.pending[this.head].id);
    const channel = findOpenChannel();
    if (channel == -1) return;
    startSound(resource, channel, this.pending[this.head].volume);
    this.head = (this.head + 1) % this.MAX_PENDING;
  }
}

class Menu {
  onSelect(index) {
    Audio.playSound(SOUND_BLOOP, VOLUME_MAX);
  }
}
