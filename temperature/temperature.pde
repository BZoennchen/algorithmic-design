
import java.util.*;


static final int START_YEAR = 1880;
static final int NUMBER_OF_DAYS = 50;
static final int STEPS = 25;
////////////////////

static final int LAST_MONTH = 7;
static final int LAST_YEAR = 2022;
static final int FIRST_MONTH = 1;
static final int FIRST_YEAR = 1880;
static final int FONT_SIZE = 40;
static final int NUMBER_OF_MONTH = 12;
static final int BACKGROUND_COLOR = 253;
static final int FONT_COLOR = 50;

static final float ANGLE_SPPED = 1.0;

LinkedList<DataPoint> coords = new LinkedList<>();
ArrayList<Integer> colors = new ArrayList<>();
int m = 2;

int year = START_YEAR;
int month = FIRST_MONTH;
int day = 1;
float maxTemperature = -1000.0f;
float minTemperature = 1000.0f;
float temperatureDiff = 0.0f;

int preX = 0;
int preY = 0;

Table data;

void loadData() {
  data = loadTable("temp.csv", "header");
  println(data.getRowCount() + " total rows in table");
  
  for(int i = FIRST_YEAR; i < LAST_YEAR; i++) {
        for(int j = FIRST_MONTH; j < LAST_MONTH; j++) {
        maxTemperature = max(maxTemperature, getTemerature(i, j));
        minTemperature = min(minTemperature, getTemerature(i, j));
      }
   }
   println(maxTemperature);
   println(minTemperature);
   temperatureDiff = maxTemperature - minTemperature;
}

void loadColors() {
  colors.add(color(0, 59, 255));
  colors.add(color(57, 97, 229));
  colors.add(color(145, 219, 255));
  colors.add(color(255, 239, 0));
  colors.add(color(255, 178, 102));
  colors.add(color(255, 153, 153));
  colors.add(color(255, 102, 102));
  colors.add(color(255, 51, 51));
}

void setup() {
  loadData();
  loadColors();
  size(800, 800); // P2D
  smooth(8);
  background(BACKGROUND_COLOR);
  translate(width/2.0f, height/2.0f);
  drawCircles();
}

// Drawing
void draw() {
  translate(width/2.0f, height/2.0f);
  for(int i = 0; i < STEPS; i++) {
      step(); 
  }
}

void step(){
    float temperature = getInterpolatedTemerature(year, month, day); 
    float radius =  toRadius(temperature);
    float theta = toAngle(month, day);
    float x = cos(theta) * radius;
    float y = sin(theta) * radius;
    
    coords.addFirst(new DataPoint(x,y,temperature,year));
    
    if(coords.size() > m) {
      coords.removeLast();
    }
  
    drawText(year, month);
    drawLines(coords);
    day += 1;
    
    if(day > NUMBER_OF_DAYS) {
      month += 1;
      day = 1;
    }
    
    if(month > NUMBER_OF_MONTH) {
      month = 1;
      year += 1;
    }
    
    if(month >= LAST_MONTH && year >= LAST_YEAR) {
        noLoop();
    }
}


void drawLines(final List<DataPoint> coords){
    float alpha = 50.0f;
    strokeWeight(5);
    stroke(0);
    noFill();
    if(coords.size() > 1) {
      DataPoint prev = null;
      for(DataPoint point : coords) {
        if(prev != null){
          int c = toColor(point.temperature);
          stroke(c, alpha);
          line(prev.x, prev.y, point.x, point.y);
        }
        prev = point;
      }
    }
}

void drawText(int year, int month) {
  textSize(FONT_SIZE);
  fill(BACKGROUND_COLOR);
  noStroke();
  rect(-width/2.0f, -height/2.0f, 160, 50);
  fill(FONT_COLOR, FONT_COLOR, FONT_COLOR);
  text(year+"/"+month, -width/2.0f+10, -height/2.0f+FONT_SIZE);
}

void drawCircles() {
    noFill();
    List<Float> temps = Arrays.asList(new Float[]{-0.5, 0.0, 0.5, 1.0, 1.5});
    textSize(FONT_SIZE/2.6);
    int alpha = 140;
    
    for(Float temp : temps) {
        stroke(FONT_COLOR, FONT_COLOR, FONT_COLOR, alpha);
        strokeWeight(0.5);
        noFill();
        float rad = toRadius(temp);
        ellipse(0, 0, rad*2, rad*2);

        noStroke();
        fill(FONT_COLOR, FONT_COLOR, FONT_COLOR, alpha);
        text(temp, -2, -rad-5);
    }
};

void resetDrawing() {
  background(BACKGROUND_COLOR);
  year = START_YEAR;
  month = FIRST_MONTH;
  day = 1;
  drawCircles();
  loop();
}

// Computation
private float toAngle(int month, int day) {
    return (ANGLE_SPPED*2*PI) / (NUMBER_OF_MONTH*NUMBER_OF_DAYS) * ((month-1)*NUMBER_OF_DAYS+(day-1));
};

private int toColor(float temperature) {
    int n = colors.size();
    int i = floor(map(temperature, minTemperature, maxTemperature, 0, n));
    i = max(i, 0);
    i = min(i, n-1);
    return colors.get(i);
};
    
private float toRadius(float temperature) {
    float maxRadius = width/2.0f-25.0f;
    float radius =  (maxRadius / temperatureDiff) * (temperature-minTemperature);
    return radius;
}

private float getTemerature(int year, int month) {
    int yearIndex = (year-FIRST_YEAR);
    //console.log(year, month, yearIndex)
    return data.getRow(yearIndex).getFloat(month);
}

private float getInterpolatedTemerature(int year, int month, int day){
    float temp1 = getTemerature(year, month);
    if(month == 12) {
        month = 1;
        year += 1;
    } else {
        month += 1;
    }
    float temp2 = getTemerature(year, month);
    float nod = NUMBER_OF_DAYS-1;
    float t1 = temp1 * (nod-(day-1))/nod;
    float t2 = temp2 * (day-1) / nod;
    return (t1 + t2);
}
