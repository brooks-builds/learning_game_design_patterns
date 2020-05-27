class EventSystem {
    constructor() {
        this.observers = [];
    }

    addObserver(observer) {
        this.observers.push(observer);
    }

    removeObserver(observer) {
        this.observers.splice(this.observers.indexOf(observer), 1);
    }

    notify(entity, event) {
        this.observers.forEach(observer => observer.onNotify(entity, event));
    }
}