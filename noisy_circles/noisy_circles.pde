float ns = 0.005;
float minRadius = 100;
int nCircles = 300;
int nPoints = 120;
float dt = 0;
float speed = 10;
float maxRadius;
float cmaxRadius = minRadius;
int maxFrameCount = 2800;

void setup() {
  size(1920, 1080, P2D);
  smooth(8);
  maxRadius = min(width, height) / 3.0f;
}

void draw() {
  background(15);
  noFill();
  stroke(255, 60);
  translate(width/2, height/2);
  rotate(dt/10.0f);
  
  if(cmaxRadius > minRadius){
    int n = int(cmaxRadius / maxRadius * nCircles);
    float delta = (cmaxRadius-minRadius) / n;
    for(int i = 0; i <  n; i++) {
      float radius = minRadius + delta*i;
      drawNoiseCircle(0, 0, radius, int(nPoints*radius/minRadius));
    }
    
    dt += ns * speed;
  }
  
  if (frameCount > maxFrameCount){
    cmaxRadius -= 1;
  } else   if (cmaxRadius < maxRadius) {
    cmaxRadius += 1;
  }
  
  if(cmaxRadius < minRadius) {
    noLoop();
  }
  
  saveFrame("./output/bubble-####.png");
  
  //saveFrame("./output/bubble-####.png");
}

void drawNoiseCircle(float xcenter, float ycenter, float radius, int n) {
  beginShape();
  float delta = TWO_PI / n;
  for(int i = 0; i < n; i++) {
    float angle = delta * i;
    //float noiseAngleX = cos(angle / 2) * radius;
    //float noiseAngleY = sin(angle / 2) * radius;
    float x_r = cos(angle) * radius;
    float y_r = sin(angle) * radius;
    float nradius = radius + (radius-minRadius) * map(noise(x_r * ns, y_r * ns, dt), 0, 1, -1, 1);
    float x = cos(angle) * nradius + xcenter;
    float y = sin(angle) * nradius + ycenter;
    vertex(x, y);
  }
  endShape(CLOSE);
}
