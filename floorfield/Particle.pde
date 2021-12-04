import java.lang.Math;

static class Particle {

  private PVector pos;
  private PVector lastPos;
  private PVector vel;
  
  public Particle(final PVector pos) {
    this.pos = pos;
    this.lastPos = pos;
    this.vel = new PVector(0,0);
  }

  public void setVel(final PVector vel){
    this.vel = vel;
  }
  
  public void setPos(final PVector pos) {
    this.lastPos = this.pos;
    this.pos = pos;
  }

  public void update(final float time) {
    PVector dx = PVector.mult(this.vel, time);
    PVector newPos = PVector.add(pos, dx);
    setPos(newPos);
  }
  
  public void update(final float time, float w, float h) {
    update(time);
    if(this.pos.x < 0) {
      this.pos.x = w;
      this.lastPos.x = w;
    }
    
    if(this.pos.y < 0) {
      this.pos.y = h;
      this.lastPos.y = h;
    }
    
    if(this.pos.x > w) {
      this.pos.x = 0;
      this.lastPos.x = 0;
    }
    
    if(this.pos.y > h) {
      this.pos.y = 0;
      this.lastPos.y = 0;
    }
  }
  
  public float getX() {
    return pos.x;
  }
  
  public float getY() {
    return pos.y;
  }
  
  public float getOldX() {
    return lastPos.x;
  }
  
  public float getOldY() {
    return lastPos.y;
  } 
  
  public static ArrayList<Particle> randomParticles(final float w, final float h, final int n) {
    ArrayList<Particle> particles = new ArrayList(n);
    for(int i = 0; i < n; i++) {
      particles.add(Particle.randomParticle(w, h));
    }
    return particles;
  }
  
  public static Particle randomParticle(final float w, final float h) {
    PVector pos = new PVector((float)Math.random() * w, (float)Math.random() * h);
    return new Particle(pos);
  }
}
