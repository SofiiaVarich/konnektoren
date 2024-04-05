class StarCreator {
  constructor() {
    this.config = {
      starAnimationDuration: 1500,
      minimumTimeBetweenStars: 250,
      minimumDistanceBetweenStars: 75,
      glowDuration: 150,
      maximumGlowPointSpacing: 10,
      minimumTimeBetweenGlows: 10,
      colors: ["249, 146, 253", "252, 254, 255"],
      sizes: ["1.4rem", "1rem", "0.6rem"],
      animations: ["fall-1", "fall-2", "fall-3"],
    };
    this.last = {
      starTimestamp: performance.now(),
      starPosition: { x: 0, y: 0 },
      mousePosition: { x: 0, y: 0 },
    };
    this.count = 0;
    this.lastGlowCreation = 0;
    this.init();
  }

  rand(min, max) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  }

  selectRandom(items) {
    return items[this.rand(0, items.length - 1)];
  }

  withUnit(value, unit) {
    return `${value}${unit}`;
  }

  calcDistance(a, b) {
    const diffX = b.x - a.x,
      diffY = b.y - a.y;
    return Math.sqrt(diffX ** 2 + diffY ** 2);
  }

  appendElement(element, duration) {
    document.body.appendChild(element);
    setTimeout(() => document.body.removeChild(element), duration);
  }

  createStar(position) {
    const star = document.createElement("span"),
      color = this.selectRandom(this.config.colors),
      animationName = this.config.animations[this.count++ % 3];

    Object.assign(star.style, {
      left: this.withUnit(position.x, "px"),
      top: this.withUnit(position.y, "px"),
      fontSize: this.selectRandom(this.config.sizes),
      color: `rgb(${color})`,
      textShadow: `0 0 1.5rem rgb(${color} / 0.5)`,
      animation: `${animationName} ${this.config.starAnimationDuration}ms linear`,
    });

    star.className = "star";
    this.appendElement(star, this.config.starAnimationDuration);
  }

  createGlow(last, current) {
    const now = performance.now();
    // Throttle glow creation to prevent performance issues
    if (now - this.lastGlowCreation < this.config.minimumTimeBetweenGlows) {
      return;
    }
    this.lastGlowCreation = now;

    const distance = this.calcDistance(last, current),
      quantity = Math.max(
        Math.floor(distance / this.config.maximumGlowPointSpacing),
        1,
      ),
      dx = (current.x - last.x) / quantity,
      dy = (current.y - last.y) / quantity;

    for (let i = 0; i < quantity; i++) {
      const glow = document.createElement("div"),
        x = last.x + dx * i,
        y = last.y + dy * i;

      Object.assign(glow.style, {
        left: this.withUnit(x, "px"),
        top: this.withUnit(y, "px"),
        position: "absolute",
        width: "5px",
        height: "5px",
        borderRadius: "50%",
        backgroundColor: "white",
        opacity: 0.5,
        // Reduce the size and opacity of glow points to lessen their visual impact
      });
      glow.className = "glow-point";
      this.appendElement(glow, this.config.glowDuration);
    }
  }

  handleMove(position) {
    const now = performance.now(),
      movedEnough =
        this.calcDistance(this.last.starPosition, position) >=
        this.config.minimumDistanceBetweenStars,
      timePassed =
        now - this.last.starTimestamp > this.config.minimumTimeBetweenStars;

    if (movedEnough || timePassed) {
      this.createStar(position);
      this.last.starTimestamp = now;
      this.last.starPosition = position;
    }

    // Create glows less frequently than stars to reduce the amount
    if (now - this.last.starTimestamp > this.config.glowDuration) {
      this.createGlow(this.last.mousePosition, position);
    }
    this.last.mousePosition = position;
  }

  init() {
    if (document.body) {
      document.addEventListener("mousemove", (e) =>
        this.handleMove({ x: e.clientX, y: e.clientY }),
      );
      document.addEventListener("touchmove", (e) =>
        this.handleMove({ x: e.touches[0].clientX, y: e.touches[0].clientY }),
      );
      document.body.addEventListener(
        "mouseleave",
        () => (this.last.mousePosition = { x: 0, y: 0 }),
      );
    }
  }
}

setTimeout(() => new StarCreator(), 1000);
