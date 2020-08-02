#version 330 core

in vec4 gl_FragCoord;
in VertexData {
    vec3 uv;
    vec3 normal;
    vec3 frag_pos;
} vertex;

out vec3 color;

uniform sampler2DArray texture_array;
uniform vec3 view_pos;
uniform vec3 fog_color;
uniform int active_lights;

uniform struct LightSource {
    vec3 color;
    vec3 world_pos;
    float absolute_intensity;
    float ambient_intensity;
    float diffuse_intensity;
    float specular_intensity;
    float specular_shininess;
}scene_lights[2];

const float FOG_DEPTH = 0.00075;

vec3 calculate_light_factor(int index) {
    vec3 ambient = scene_lights[index].color * scene_lights[index].ambient_intensity;
    vec3 diffuse = vec3(0., 0., 0.);
    vec3 specular = vec3(0., 0., 0.);

    vec3 light_dir = scene_lights[index].world_pos - vertex.frag_pos;
    float distance = length(light_dir);
    distance *= distance;

    light_dir = normalize(light_dir);

    float lambert = max(0., dot(vertex.normal, light_dir));

    if (lambert > 0.) {
        diffuse = scene_lights[index].color * lambert * scene_lights[index].diffuse_intensity * scene_lights[index].absolute_intensity / distance;
        vec3 view_dir = normalize(view_pos - vertex.frag_pos);
        vec3 half_dir = normalize(light_dir + view_dir);
        float spec_angle = max(dot(half_dir, vertex.normal), 0.);
        float spec_strength = pow(spec_angle, scene_lights[index].specular_shininess);
        specular = scene_lights[index].color * spec_strength * scene_lights[index].specular_intensity * scene_lights[index].absolute_intensity / distance;
    }
    return ambient + diffuse + specular;
}

float calculate_fog_factor() {
    float dist = gl_FragCoord.z / gl_FragCoord.w;
    return exp(-pow((dist * FOG_DEPTH), 2));
}

void main() {
    float slope = max(0, dot(vertex.normal, vec3(0., 0., 1.)));
	float height = vertex.frag_pos.z;
    //color = texture(texture_array, vertex.uv).rgb;

	color = vec3(0.2, 0.6, 0.2);
	if (height > 140.){
		color = mix(vec3(0.8, 0.8, 0.8), color, 1 - min(1., (height - 140.)/40.));
	}

	float rock_factor = min(1., (max(0., (height - 120)) / 10.));

	color = mix(vec3(0.3, 0.3, 0.3), color, pow(slope, 4.));

    vec3 light_factor = vec3(0., 0., 0.);
    for (int i = 0; i < active_lights; i++) {
        light_factor += calculate_light_factor(i);
        if (light_factor.x >= 1. && light_factor.y >= 1. && light_factor.z >= 1.) {
            light_factor = vec3(1., 1., 1.);
            break;
        }
    }
    color *= light_factor;

    float fog_factor = calculate_fog_factor();
    color = mix(fog_color * scene_lights[0].absolute_intensity / 1e8, color, fog_factor);
}
