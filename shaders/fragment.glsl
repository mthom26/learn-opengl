in vec3 v_col;

out vec3 frag_color;

uniform float time;

void main() {
  frag_color = vec3(time, v_col.y, v_col.z);
}
