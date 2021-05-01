let amount = 0
let scale = 1
const PHI = 1.618

function setup() {
  createCanvas(500, 500)
};

function draw() {
  strokeWeight(1)
  background(255)

  let px = width/2
  let py = height/2

  for (let t = 0; t < amount; t+=0.01) {
    const r = PHI * exp(0.30635*t)
  
    const x = width/2 + ((r * cos(t)) * scale)
    const y = height/2 + ((r * sin(t)) * scale)

    if (px != width/2 && py != height/2) {
      line(px, py, x, y)
    }

    px = x
    py = y
  }

  amount+= 0.05;
  if (amount > 10) {
    scale *= 0.995;
  }
}
