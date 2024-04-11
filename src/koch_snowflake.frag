#version 410

uniform vec2 u_screenSize; // screen size in pixels

layout(location = 0) out vec4 o_colour;	// output to colour buffer

float TAU = 6.28318530718;
float scale = 1.;

// ref: https://www.shadertoy.com/view/ll3XRn
float ifs(vec2 p, float angle, float scale, int n) {
   float s = sin(angle), c = cos(angle);
   mat2 r = mat2(c, -s, s, c);
   for (int i = 0; i < n; i++) {
      p = r*p + p;
      p = -p;
      p.y = scale - abs(p.y);
   }
   return p.x;
}

void main() {
   vec2 p = gl_FragCoord.xy / u_screenSize.x - 0.5;
   p = abs(p); // fold
   o_colour += round(ifs(p, TAU/6, scale, 13));
}