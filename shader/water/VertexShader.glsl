#version 330 core

layout(location = 0) in vec3 vertex_pos;
layout(location = 1) in vec3 vertex_uv;
layout(location = 2) in vec3 vertex_normal;

uniform mat4 mvp;
uniform mat4 model;
uniform vec3 view_pos;
uniform float dudv_offset;
uniform sampler2D dudv_map;

out VertexData {
	vec2 uv;
	vec3 normal;
	vec3 world_pos;
} vertex;

void main() {
	gl_Position = mvp * vec4(vertex_pos, 1.);
	vertex.uv = (vertex_pos.xy + 1.)/2. * 1024.;
	vertex.normal = vec3(0., 0., 1.);
	vertex.world_pos = vec3(model * vec4(vertex_pos, 1.));

}
