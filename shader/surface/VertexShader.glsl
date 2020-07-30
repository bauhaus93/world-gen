#version 330 core

layout(location = 0) in vec3 vertex_pos;
layout(location = 1) in vec3 vertex_uv;
layout(location = 2) in vec3 vertex_normal;

out VertexData {
  vec3 uv;
  vec3 normal;
  vec3 frag_pos;
} vertex;

uniform mat4 mvp;
uniform mat4 model;

void main() {
  gl_Position = mvp * vec4(vertex_pos, 1.);
  vertex.uv = vertex_uv;
  vertex.normal = vertex_normal;
  vertex.frag_pos = vec3(model * vec4(vertex_pos, 1.));
}
