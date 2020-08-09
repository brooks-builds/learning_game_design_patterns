class EventSystem {
  constructor() {
    this.listeners = [];
  }

  notify(data) {
    this.listeners.forEach((callback) => callback(data));
  }

  registerListener(callback) {
    this.listeners.push(callback);
  }
}
