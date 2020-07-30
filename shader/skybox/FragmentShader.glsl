#version 330 core

in VertexData {
    vec3 uv;
} vertex;

out vec3 color;

uniform samplerCube cube_texture;
uniform float light_level;

void main() {
    float light_factor = min(1., light_level / 1e8);
    color = light_factor * texture(cube_texture, vertex.uv).rgb;
}
