int n = 50000;

float a = 0;
float b = -1.50;
float c = 1.1;
float d = -2.50;
boolean addition = true;

void setup() {
  size(1024, 768, P2D);
  background(100);
  stroke(0, 70);
  fill(0, 70);
}

void draw() {
  background(255);
  translate(width/2, height/2);
  
  float x = 0;
  float y = 0;
  
  for(int i = 0; i < n; i++){
    float x_r = sin(a * y) - cos(b * x);
    float y_r = sin(c * x) - cos(d * y);
    point(x_r * width/5, y_r * height/5);
    x = x_r;
    y = y_r;
  }

  if(a > 2 * PI){
    addition = false;
  }
    
  if(a <= 0) {
    addition = false;
  }
    
  if(addition){
    a += 0.01;
    b += 0.01;
  } else {
    a -= 0.01;
    b -= 0.01;
  }
  println(frameRate);
  /*if(a < 0) {
    noLoop();
  } else {
    saveFrame("output/dong_####.png");
  }*/
}
    
