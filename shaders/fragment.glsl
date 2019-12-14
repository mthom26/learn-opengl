in vec3 v_frag_pos;
in vec3 v_normal;

out vec4 frag_color;

uniform vec3 object_color;
uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 cam_pos;

void main() {
  // Get ambient
  vec3 ambient = 0.1 * light_color;

  // Get diffuse
  vec3 norm = normalize(v_normal);
  vec3 light_dir = normalize(light_pos - v_frag_pos);
  float diff = max(dot(norm, light_dir), 0.0);
  vec3 diffuse = diff * light_color;

  // Get specular
  float spec_strength = 0.5;
  vec3 view_dir = normalize(cam_pos - v_frag_pos);
  vec3 reflect_dir = reflect(-light_dir, norm);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
  vec3 specular = spec_strength * spec * light_color;

  // Final fragment color
  vec3 result = (ambient + diffuse + specular) * object_color;
  frag_color = vec4(result, 1.0);
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