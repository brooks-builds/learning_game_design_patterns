class Player {
    constructor(location, jumpOverObstacleEvent) {
        this.initialLocation = location.copy();
        this.reset();
        this.width = 5;
        this.height = 20;
        this.jumpForce = createVector(0, -10);
        this.jumpOverObstacleEvent = jumpOverObstacleEvent

        this.jumpOverObstacleEvent.addObserver(this)
    }

    render() {
        fill(0);
        rect(this.location.x, this.location.y, this.width, this.height);
    }

    jump() {
        if (!this.isJumping) {
            this.applyForce(this.jumpForce);
            this.isJumping = true;
        }
    }

    applyForce(force) {
        this.acceleration.add(force);
    }

    hitGround(groundLocation) {
        if (this.location.y + this.height > groundLocation) {
            this.location.y = groundLocation - this.height;
            this.velocity.y = 0;
            this.isJumping = false;
        }
    }

    update() {
        this.velocity.add(this.acceleration);
        this.location.add(this.velocity);
        this.acceleration.mult(0);
    }

    isHitting(obstacle) {
        return this.location.x < obstacle.location.x + obstacle.width &&
            this.location.x + this.width > obstacle.location.x &&
            this.location.y < obstacle.location.y + obstacle.height &&
            this.location.y + this.height > obstacle.location.y;
    }

    incrementScore() {
        this.score += 1;
    }

    reset() {
        this.score = 0;
        this.location = this.initialLocation.copy();
        this.velocity = createVector(0, 0);
        this.acceleration = createVector(0, 0);
        this.isJumping = true;
    }

    onNotify(entity, event) {
        if (event == EVENT_JUMPED_OVER_OBSTACLE) {
            entity.incrementScore();
        }
    }
}