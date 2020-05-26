class Physics extends Subject {
    constructor() {
        this.gravity = createVector(0, 1);
    }

    updateEntity(entity) {
        const wasOnSurface = entity.isOnSurface();

        entity.accelerate(this.gravity);
        entity.update();

        if (wasOnSurface && !entity.isOnSurface()) {
            notify(entity, EVENT_START_FALL);
        }
    }
}

class Observer extends Subject {
    constructor() {
        this.next = null;
    }

    onNotify(entity, event) {

    }
}

class Achievements {
    constructor(entityFellEvent) {
        this.heroIsOnBridge;
        this.entityFellEvent = entityFellEvent
    }

    onNotify(entity, event) {
        switch (event) {
            case EVENT_ENTITY_FELL:
                if (entity.isHero() && this.heroIsOnBridge) {
                    unlock(ACHIEVEMENT_FELL_OFF_BRIDGE);
                }
                break;
        }
    }

    unlock(achievement) {
        // do the unlock
    }
}

class Subject {
    constructor() {
        this.head = null;
    }

    addObserver(observer) {
        observer.next = this.head;
        this.head = observer;
    }

    removeObserver(observer) {
        if (this.head == observer) {
            this.head = observer.next;
            return;
        };

        let current = this.head;
        while (current != null) {
            if (current.next == observer) {
                current.next = observer.next;
                observer.next = null;
                return;
            }

            current = current.next;
        }
    }

    notify(entity, event) {
        let observer = this.head;

        while (observer) {
            observer.onNotify(entity, event);
            observer = observer.next;
        }
    }
}

const entityFell = new Event();