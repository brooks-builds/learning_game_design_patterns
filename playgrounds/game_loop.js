// version 1

const MS_PER_FRAME = 1000 / 60;
const MS_PER_UPDATE = 1000 / 300;

function gameLoop() {
  const now = Date.now();
  processInput();
  update();
  render();

  setTimeout(gameLoop, now + MS_PER_FRAME - Date.now());
}

// start game loop
gameLoop();

// version 2

let lastTime = Date.now();
while (true) {
  const current = Date.now();
  const elapsed = current - lastTime;
  processInput();
  update(elapsed);
  render();
  lastTime = current;
}

// version 3

let previous = Date.now();
let lag = 0;
while (true) {
  const current = Date.now();
  const elapsed = current - previous;
  previous = current;
  lag += elapsed;

  processInput();

  while (lag >= MS_PER_UPDATE) {
    update();
    lag -= MS_PER_UPDATE;
  }

  render(lag / MS_PER_UPDATE);
}
