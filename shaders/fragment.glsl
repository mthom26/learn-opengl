out vec4 frag_color;

uniform vec3 object_color;
uniform vec3 light_color;

void main() {
  frag_color = vec4(object_color * light_color, 1.0);
}

// Old
// in vec3 v_col;
// in vec2 v_tex_coord;

// out vec4 frag_color;

// // uniform float time;
// uniform sampler2D tex;
// uniform sampler2D tex_smiley;

// void main() {
//   // frag_color = vec3(time, v_col.y, v_col.z);
//   // frag_color = texture(tex, v_tex_coord) * vec4(v_col, 0.2);

//   frag_color = mix(
//     texture(tex, v_tex_coord),
//     texture(tex_smiley, v_tex_coord),
//     0.2
//   );
// }