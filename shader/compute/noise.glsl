#version 430

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(rg32f, binding = 0) uniform image1D grid_origins;
layout(r32f, binding = 1) uniform image3D heightmaps;

void main() {
	vec3 pixel = vec3(0., 0., 0.);

	ivec2 pixel_coord = ivec2(gl_GlobalInvocationID.xy);


	imageStore(img_output, pixel_coord, pixel);
}

