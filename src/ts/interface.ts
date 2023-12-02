interface Area {
  area(): number;
}

class Rectangle implements Area {
  constructor(
    private x: number,
    private y: number,
    private width: number,
    private heigh: number,
  ) {}

  area(): number {
    return this.width * this.heigh;
  }
}

class Circle implements Area {
  constructor(
    private x: number,
    private y: number,
    private radius: number,
  ) {}

  area(): number {
    return this.radius * this.radius * Math.PI;
  }
}
