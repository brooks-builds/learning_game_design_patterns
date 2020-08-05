class Camera {
  constructor(location, velocity = createVector(1, 0), width, height) {
    this.location = location;
    this.velocity = velocity;
    this.width = width;
    this.height = height;
  }
}
