function setup() {
    createCanvas(770, 700);
    player = new Player(createVector(50, 25));
    gravity = createVector(0, 1);
}

function draw() {
    clear();
    fill(0);
    rect(0, height - 5, width, 5);
    player.render();

    if (keyIsDown(KEYCODE_SPACE)) {
        console.log('space pressed')
        player.jump();
    }
    player.update();
    player.applyForce(gravity);
    player.hitGround(height - 5);
}

let player;
const KEYCODE_SPACE = 32;
let gravity;