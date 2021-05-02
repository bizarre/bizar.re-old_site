let f = 0;
let amount = 0

function setup() {
  createCanvas(500, 500)
};

function draw() {
  strokeWeight(1)
  background(255)

  let px = width/2
  let py = height/2

  for (let r = 0; r < amount; r+=5) {
    const o = PI * 2 * f * r;
  
    const x = width/2 + r * cos(o)
    const y = height/2 + r * sin(o)
  
    line(px, py, x, y)

    px = x
    py = y
  }

  f += 0.0001
  amount+= 0.1
}
