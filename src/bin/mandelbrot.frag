#version 410

uniform vec2 u_screenSize; // screen size in pixels

layout(location = 0) out vec4 o_colour;	// output to colour buffer

int ITERATIONS = 100; // Number of iterations for the function to be run (must be greater than 0)

vec2 mandelbrot(vec2 z, vec2 c) {
   return vec2(pow(z.x, 2) - pow(z.y, 2), 2 * z.x * z.y) + c;
}

void main() {
   vec2 c = gl_FragCoord.xy/u_screenSize * 2 - 1;

   vec2 z = c;
   for (int i = 0; i < ITERATIONS-1; i++) {
      z = mandelbrot(z, c);
   }

   if (length(z) <= 1) {
      o_colour = vec4(0.0, 0.0, 0.0, 1.0);
   } else {
      o_colour = vec4(1.0, 1.0, 1.0, 1.0);
   }
}