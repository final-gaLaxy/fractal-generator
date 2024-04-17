#version 410

in vec4 a_position;	/* vertex position */

uniform mat4 u_mvpMatrix;

out vec4 v_position;

void main() {
    vec4 p = u_mvpMatrix * a_position;
    gl_Position = a_position;
    v_position = p;
}

