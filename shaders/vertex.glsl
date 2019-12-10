in vec3 pos;
in vec3 col;

out vec3 v_col;

void main() {
  v_col = col;
  gl_Position = vec4(pos, 1.0);
}
