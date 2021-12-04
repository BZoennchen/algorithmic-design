n = 10000

a = 0
b = -1.50
c = 1.1
d = -2.50
addition = True

def setup():
  size(1024, 768, P2D)
  background(100)
  #frameRate(25)
  stroke(0, 70)
  fill(0, 70)
  #background(255)


def draw():
    background(255)
    translate(width/2, height/2)
    global a, b
    global addition
    
    x = 0
    y = 0
    
    #multiplier = 2 * PI / min(abs(d),min(abs(c),min(abs(a),abs(b))))
    
    for i in range(n):
        x_r = sin(a * y) - cos(b * x)
        y_r = sin(c * x) - cos(d * y)
        point(x_r * width/5, y_r * height/5)
        #rad = random(0,1)*3
        #ellipse(x_r * width/5, y_r * height/5, rad, rad)
        x = x_r
        y = y_r
    
    if a > 2 * PI:
        addition = False
    
    if a <= 0:
        addition = True
    
    if addition:
        a += 0.01
        b += 0.01
    else:
        a -= 0.01
        b -= 0.01
    
    println(frameRate)
    #if a < 0:
    #    noLoop()
    #else:
    #   saveFrame("output/dong_####.png")
