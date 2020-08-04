#version 330 core

in vec4 gl_FragCoord;
in VertexData {
  vec2 uv;
  vec3 normal;
  vec3 world_pos;
} vertex;

uniform vec3 view_pos;
uniform int active_lights;
uniform sampler2D normal_map;
uniform sampler2D dudv_map;
uniform float dudv_offset;

out vec3 color;

uniform struct LightSource {
    vec3 color;
    vec3 world_pos;
    float absolute_intensity;
    float ambient_intensity;
    float diffuse_intensity;
    float specular_intensity;
    float specular_shininess;
}scene_lights[2];

vec3 calculate_light_factor(int i, vec3 normal) {
    vec3 ambient = scene_lights[i].color * scene_lights[i].ambient_intensity;
    vec3 diffuse = vec3(0., 0., 0.);
    vec3 specular = vec3(0., 0., 0.);

    vec3 light_dir = scene_lights[i].world_pos - vertex.world_pos;
    float dist = length(light_dir);
    dist *= dist;

    light_dir = normalize(light_dir);

    float lambert = max(0., dot(light_dir, normal));

    if (lambert > 0.) {
        diffuse = scene_lights[i].color * lambert * scene_lights[i].diffuse_intensity * scene_lights[i].absolute_intensity / dist;
        vec3 view_dir = normalize(view_pos - vertex.world_pos);
		vec3 reflect_dir = reflect(-light_dir, normal);
        float spec_angle = max(dot(reflect_dir, view_dir), 0.);
        float spec_strength = pow(spec_angle, scene_lights[i].specular_shininess);
        specular = scene_lights[i].color * spec_strength * scene_lights[i].specular_intensity * scene_lights[i].absolute_intensity / dist;
    }
    return ambient + diffuse + specular;
}

void main() {
	color = vec3(0.2, 0.2, 0.8);
	vec2 tex_coords = texture2D(dudv_map, vec2(vertex.uv.x + dudv_offset, vertex.uv.y)).rg * 0.1;
	tex_coords = tex_coords + vec2(tex_coords.x, tex_coords.y + dudv_offset);
	tex_coords = (texture2D(dudv_map, tex_coords).rg * 2.) / 10.;

	vec3 normal_map_color = texture2D(normal_map, tex_coords).rgb;
	vec3 normal = normalize(vec3(
			normal_map_color.r * 2. + 1.,
			normal_map_color.b * 2.6,
			normal_map_color.g * 2. + 1.));

	vec3 light_factor = vec3(0., 0., 0.);
    for (int i = 0; i < active_lights; i++) {
        light_factor += calculate_light_factor(i, normal);
        if (light_factor.x >= 1. && light_factor.y >= 1. && light_factor.z >= 1.) {
            light_factor = vec3(1., 1., 1.);
            break;
        }
    }
    color *= light_factor;


}
