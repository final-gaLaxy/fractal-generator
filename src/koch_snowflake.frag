#version 410

uniform vec2 u_screenSize; // screen size in pixels

layout(location = 0) out vec4 o_colour;	// output to colour buffer

int MAX_ITERATIONS = 4; // Number of iterations for the function to be run (must be greater than 0)

bool in_triangle(vec2 p, vec2 a, vec2 b, vec2 c) {
   /*
      Determines if a point p is in triangle ABC by its barycentric coordinates

      P = (1-v-w) A + v B + w C
        = A + v(B-A) + w(C-A)

      Solve with v0 = b-a, v1 = c-a, v2 = p-a
      P - A = v(B-A) + w(C-A)
      v2 = v v0 + w v1

      create simultaneous equations by taking the dot product of v0, and v1 on each side
      (1) v2.v0 = v (v0.v0) + w(v1.v0)
      (2) v2.v1 = v (v0.v1) + w(v1.v1)

      Use Craner's Rule to find solutions of v and w (u = 1 - v - w)
      if v<0, w<0 or v+w>1, then p is outside triangle ABC
   */
   vec2 v0 = b-a, v1 = c-a, v2 = p-a;

   float d00 = dot(v0, v0);
   float d01 = dot(v0, v1);
   float d02 = dot(v0, v2);
   float d11 = dot(v1, v1);
   float d12 = dot(v1, v2);

   // Cramer's Rule
   float denom = d00 * d11 - d01 * d01;

   // only use denom to check within range
   float v = d02 * d11 - d01 * d12;
   float w = d00 * d12 - d02 * d01;

   return !(v*denom < 0 || w*denom < 0 || abs(v + w) > denom);
}

float sin60 = sqrt(3)/2; // sin(60deg)

bool koch_curve(in vec2 p, inout vec2 segStart, inout vec2 segEnd) {
   // Ignore if outside likely area
   vec2 seg = segEnd - segStart;
   vec2 t = seg/2 + segStart + vec2(segStart.y - segEnd.y, segEnd.x - segStart.x) * sin60;
   if (!in_triangle(p, segStart, segEnd, t)) {
      return false;
   }

   vec2 Q = segEnd - seg * dot(segEnd - p, seg)/dot(seg,seg);

   float len = distance(segStart, segEnd)/3 * sin60;
   if (distance(Q, p) >= len) {
      return false;
   }

   // Create a equilateral triangle from segment
   vec2 a =   seg/3 + segStart;
   vec2 b = 2*seg/3 + segStart;
   vec2 c =   seg/2 + segStart + vec2(segStart.y - segEnd.y, segEnd.x - segStart.x)/3 * sin60;

   if (in_triangle(p, a, b, c)) {
      return true;
   }
   else {
      // check if left or right side
      float side = dot(seg, vec2(p.x - c.x, p.y - c.y));
      if(side < 0) {
         // Left
         if (distance(c, p) < distance(segStart, p)) {
            segStart = a;
            segEnd = c;
         }
         else {
            segEnd = a;
         }
      } else {
         // Right
         if (distance(c, p) < distance(segEnd, p)) {
            segStart = c;
            segEnd = b;
         }
         else {
            segStart = b;
         }
      }
   }
   return false;
}

vec2 center = vec2(0,sin60/4);
float size = 1;

void main() {
   vec2 p = gl_FragCoord.xy/u_screenSize * 2 - 1;

   vec2 a = vec2(-0.5,-sin60/2) * size + center,
        b = vec2(0, sin60/2) * size + center,
        c = vec2(0.5, -sin60/2) * size + center;

   o_colour = vec4(1.0, 1.0, 1.0, 1.0);

   // check in initial triangle
   if (in_triangle(p, a, b, c)) {
      o_colour = vec4(0.0, 0.0, 0.0, 1.0);
   } else {
      // determine triangle side
      float side = dot(vec2(b.y - c.y, c.x - b.x), vec2(p.x - b.x, p.y - b.y)); // right side
      if (side > 0) {
         a = b;
         b = c;
      }
      side = dot(vec2(c.y - a.y, a.x - c.x), vec2(p.x - c.x, p.y - c.y));
      if (side > 0) {
         b = a;
         a = c;
      }

      bool result = false;
      for (int i = 0; i < MAX_ITERATIONS && !result; i++) {
         result = koch_curve(p, a, b);
      }

      if (result) {
         o_colour = vec4(0.0, 0.0, 0.0, 1.0);
      }
   }
   /*

   */
}