class Player {
    constructor(location) {
        this.width = 5;
        this.height = 20;
        this.location = location;
    }

    render() {
        fill(0);
        rect(this.location.x, this.location.y, this.width, this.height);
    }
}