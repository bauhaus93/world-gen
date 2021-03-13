#version 330 core

layout(location = 0) in vec3 vertex_pos;

out VertexData {
  vec3 normal;
  vec3 frag_pos;
} vertex;

uniform sampler2D heightmap;
uniform mat4 mvp;
uniform mat4 model;
uniform int chunk_size;

void main() {
  vec4 map_texel = texture2D(heightmap, vertex_pos.xy / chunk_size);
  vec4 transformed_vertex = vec4(vertex_pos.xy, map_texel.r, 1.);
  gl_Position = mvp * transformed_vertex;
  vertex.normal = map_texel.gba;
  vertex.frag_pos = vec3(model * transformed_vertex);
}
