in vec3 pos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main() {
  gl_Position = proj * view * model * vec4(pos, 1.0);
}

// Old
// in vec3 pos;
// in vec3 col;
// in vec2 tex_coord;

// out vec3 v_col;
// out vec2 v_tex_coord;

// uniform mat4 model;
// uniform mat4 view;
// uniform mat4 proj;

// void main() {
//   v_col = col;
//   v_tex_coord = tex_coord;
//   gl_Position = proj * view * model * vec4(pos, 1.0);
// }
