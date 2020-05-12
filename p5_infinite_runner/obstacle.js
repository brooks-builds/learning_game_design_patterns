class Obstacle {
    constructor(location) {
        this.initialLocation = location.copy();
        this.initialize();
        this.width = 15;
        this.height = 15;
        this.increaseSpeedBy = 0.1;
    }

    render() {
        fill(0);
        triangle(
            this.location.x,
            this.location.y,
            this.location.x + this.width,
            this.location.y,
            this.location.x + (this.width / 2),
            this.location.y + (this.height)
        );
    }

    update(velocity, player) {
        this.location.add(velocity);
        this.reset(player);
    }

    reset(player) {
        if (this.location.x + this.width < 0) {
            this.location.x = width + 5;
            player.incrementScore();
        }
    }

    initialize() {
        this.location = this.initialLocation.copy();
    }
}