in vec3 pos;
in vec3 col;
in vec2 tex_coord;

out vec3 v_col;
out vec2 v_tex_coord;

void main() {
  v_col = col;
  v_tex_coord = tex_coord;
  gl_Position = vec4(pos, 1.0);
}
