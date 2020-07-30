#version 330 core

layout(location = 0) in vec3 vertex_pos;

out VertexData {
  vec3 uv;
} vertex;

uniform mat4 mvp;

void main() {
  gl_Position = mvp * vec4(vertex_pos, 1.);
  vertex.uv = vertex_pos;
}
