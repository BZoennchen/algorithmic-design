
final int n_points = 100000;
final float noiseScale = 0.0410150f;
final float znoiseDelta = 0.000f;
float znoise = 0.0f;
final int maxFrames = 50;
final float speed = 0.8f;
ArrayList<Particle> particles;
ArrayList<Color> colors;
PGraphics pg;
int[][] baseColors = new int[][]{
  new int[]{23, 63, 95},
  new int[]{32, 99, 155},
  new int[]{60, 174, 163},
  //new int[]{246, 213, 92}
  new int[]{237, 85, 59}
};


void setup() {
  size(1600, 900, P2D);
  particles = Particle.randomParticles(width, height, n_points);
  colors = new ArrayList(particles.size());
  
  for(Particle particle : particles) {
    int indexX = int(particle.getX() / (width/20.0)) % baseColors.length;
    int indexY = int(particle.getY() / (height/20.0)) % baseColors.length;
    int index = (indexX + indexY) % baseColors.length;
    int[] rgb = baseColors[index];
    Color c = new Color(rgb[0], rgb[1], rgb[2]);
    colors.add(c);
  }
  
  pg = createGraphics(width, height, P2D);
  background(255);
}

void draw() {
  //
  update();
  pg.beginDraw();
  //pg.background(255, 10);
  for(int i = 0; i < particles.size(); i++) {
    Particle particle = particles.get(i);
    Color c = colors.get(i);
    //pg.stroke(c.r, c.g, c.b, 255);
    pg.stroke(0, 1);
    pg.strokeWeight(1);
    pg.line(particle.getX(), particle.getY(), particle.getOldX(), particle.getOldY());
  }
  pg.endDraw();
  image(pg, 0, 0);
  znoise += znoiseDelta;
  
  
  if(frameCount > maxFrames){
    noLoop();
    stroke(0);
    fill(255);
    strokeWeight(5);
    int border = 175;
    rect(border, border, width-2*border, height-2*border, 60);
    save("./output/background.png");
  } 
}

void update() {
  for(Particle particle : particles) {
    update(particle); 
  }
}

void update(final Particle particle) {
  float xnoise = abs(particle.getX()-width/2) * noiseScale;
  float ynoise = abs(particle.getY()-height/2) * noiseScale;
  float radnoise = noise(xnoise, ynoise, znoise);
  float rad = map(radnoise, 0, 1, 0, TWO_PI);
  PVector vel = new PVector(1,0);
  vel.rotate(rad);
  
  particle.setVel(vel);
  particle.update(speed, width, height);
}
