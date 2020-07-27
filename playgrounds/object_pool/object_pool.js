class Particle {
  constructor() {
    this.framesLeft = null;
    this.x = null;
    this.y = null;
    this.xVelocity = null;
    this.yVelocity = null;
    this.next = null;
  }

  init(x, y, xVelocity, yVelocity, lifetime) {
    this.x = x;
    this.y = y;
    this.xVelocity = xVelocity;
    this.yVelocity = yVelocity;
    this.framesLeft = lifetime;
  }

  animate() {
    if (this.inUse) return;

    this.framesLeft -= 1;
    this.x += this.xVelocity;
    this.y += this.yVelocity;

    return this.isDead;
  }

  get inUse() {
    return this.framesLeft > 0;
  }

  get isDead() {
    return this.framesLeft === 0;
  }

  get next() {
    return this.next;
  }

  set next(nextParticle) {
    this.next = nextParticle;
  }
}

class ParticlePool {
  constructor() {
    this.poolSize = 100;
    this.particles = new Array(this.poolSize);
    this.particles.forEach((_particle, index, particles) => {
      particles[index] = new Particle();
    });
    this.firstAvailable = this.particles[0];
    for (let index = 0; index < this.particles.length - 1; index += 1) {
      this.particles[index].next = this.particles[index + 1];
    }
    this.particles[this.particles.length - 1].next = null;
  }

  create(x, y, xVelocity, yVelocity, lifetime) {
    if (!this.firstAvailable) throw new Error("Pool is full");

    const particle = this.firstAvailable;
    this.firstAvailable = particle.next;
    particle.init(x, y, xVelocity, yVelocity, lifetime);
  }

  animate() {
    this.particles.forEach((particle) => {
      if (particle.animate()) {
        particle.next = this.firstAvailable;
        this.firstAvailable = particle;
      }
    });
  }
}
