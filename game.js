function setup() {
    createCanvas(770, 700);
    player = new Player(createVector(50, height - 25));
}

function draw() {
    fill(0);
    rect(0, height - 5, width, 5);
    player.render();
}

let player;