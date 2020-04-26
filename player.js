class Player {
    constructor(location) {
        this.width = 5;
        this.height = 20;
        this.location = location;
        this.velocity = createVector(0, 0);
        this.accelecation = createVector(0, 0);
        this.jumpForce = createVector(0, -10);
        this.isJumping = false;
        this.score = 0;
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
        this.accelecation.add(force);
    }

    hitGround(groundLocation) {
        if (this.location.y + this.height > groundLocation) {
            this.location.y = groundLocation - this.height;
            this.velocity.y = 0;
            this.isJumping = false;
        }
    }

    update() {
        this.velocity.add(this.accelecation);
        this.location.add(this.velocity);
        this.accelecation.mult(0);
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
}