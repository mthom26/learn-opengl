in vec3 v_frag_pos;
in vec3 v_normal;

out vec4 frag_color;

uniform vec3 mat_amb;
uniform vec3 mat_diff;
uniform vec3 mat_spec;
uniform float mat_shininess;

uniform vec3 light_amb;
uniform vec3 light_diff;
uniform vec3 light_spec;
uniform vec3 light_pos;
uniform vec3 cam_pos;

void main() {
  // Get ambient
  vec3 ambient = mat_amb * light_amb;

  // Get diffuse
  vec3 norm = normalize(v_normal);
  vec3 light_dir = normalize(light_pos - v_frag_pos);
  float diff = max(dot(norm, light_dir), 0.0);
  vec3 diffuse = diff * mat_diff * light_diff;

  // Get specular
  vec3 view_dir = normalize(cam_pos - v_frag_pos);
  vec3 reflect_dir = reflect(-light_dir, norm);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), mat_shininess);
  vec3 specular = spec * mat_spec * light_spec;

  // Final fragment color
  vec3 result = ambient + diffuse + specular;
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