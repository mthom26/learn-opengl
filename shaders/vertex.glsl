in vec3 pos;
in vec3 col;
in vec2 tex_coord;

out vec3 v_col;
out vec2 v_tex_coord;

uniform mat4 trans;

void main() {
  v_col = col;
  v_tex_coord = tex_coord;
  gl_Position = trans * vec4(pos, 1.0);
}
