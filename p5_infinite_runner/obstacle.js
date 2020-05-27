class Obstacle {
    constructor(location, playerJumpedOverObstacleEvent) {
        this.initialLocation = location.copy();
        this.initialize();
        this.width = 15;
        this.height = 15;
        this.increaseSpeedBy = 0.1;
        this.playerJumpedOverObstacleEvent = playerJumpedOverObstacleEvent;
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
        const isToRightOfPlayer = this.isRightOfPlayer(player);
        this.location.add(velocity);

        if (isToRightOfPlayer && this.isLeftOfPlayer(player)) {
            this.playerJumpedOverObstacleEvent.notify(player, EVENT_JUMPED_OVER_OBSTACLE);
        }
        this.reset();
    }

    reset() {
        if (this.location.x + this.width < 0) {
            this.location.x = width + 5;
        }
    }

    initialize() {
        this.location = this.initialLocation.copy();
    }

    isRightOfPlayer(player) {
        return this.location.x > player.location.x + player.width / 2;
    }

    isLeftOfPlayer(player) {
        return this.location.x < player.location.x + player.width / 2;
    }
}