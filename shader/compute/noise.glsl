#version 430

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(rgba32f, binding = 0) readonly uniform image1D grid_origins;
layout(rgba32f, binding = 1) writeonly uniform image2DArray heightmaps;

void main() {
	float height = 0.;

	ivec3 coord = ivec3(gl_GlobalInvocationID);

	vec2 grid_origin = imageLoad(grid_origins, coord.z).rg;
	height = 100. * (coord.x  + coord.y);

	imageStore(heightmaps, coord, vec4(height, 0., 0., 0.));
}

