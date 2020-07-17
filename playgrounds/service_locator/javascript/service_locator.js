class Audio {
  constructor() {}

  playSound(soundID) {
    throw new Error("must implement");
  }

  stopSound(soundId) {
    throw new Error("must implement");
  }

  stopAllSounds() {
    throw new Error("must implement");
  }
}

class ConsoleAudio extends Audio {
  constructor() {
    super();
  }

  playSound(soundId) {
    // some implementation goes here
  }

  stopSound(soundId) {
    // some implementation goes here
  }

  stopAllSounds() {
    // some implementation goes here
  }
}

class NullAudio extends Audio {
  constructor() {
    super();
  }

  playSound(soundId) {
    this.log("playSound " + "called");
  }

  stopSound(soundId) {
    this.log("stopSound " + "called");
  }

  stopAllSounds() {
    this.log("stopAllSounds " + "called");
  }

  log(message) {
    console.error("null audio service used", message);
  }
}

class LoggedAudio extends Audio {
  constructor(wrappedAudio) {
    super();
    this.wrappedAudio = wrappedAudio;
  }

  playSound(soundId) {
    this.log("playSound");
    this.wrappedAudio.playSound();
  }

  stopSound(soundId) {
    this.log("stopSound");
    this.wrappedAudio.stopSound();
  }

  stopAllSounds() {
    this.log("stopAllSounds");
    this.wrappedAudio.stopAllSounds();
  }

  log(message) {
    console.log(message);
    // send message to database
    // hit api with something that happened
  }
}

class Locator {
  constructor(nullSystem) {
    this.nullSystem = nullSystem;
    this.service = new NullAudio();
  }

  getAudio() {
    return this.service;
  }

  provide(service = this.nullSystem) {
    this.service = service;
  }
}

// setup
const audioLocator = new Locator(new NullAudio());
// const controllerLocator = new Locator(new NullController());
audioLocator.provide(new Audio());
enableAudioLogging();

// main loop
const audioService = audioLocator.getAudio();

function enableAudioLogging(audioLocator) {
  audioLocator.provide(new LoggedAudio(new ConsoleAudio()));
}
