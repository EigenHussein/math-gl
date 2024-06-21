#version 460 core

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct vertex {
    vec4 position;
    vec4 color;
};

// struct vertex_pack {
//     vertex pack[4];
// };

layout(std430, binding = 0) buffer InputBuf {
vertex vertices_in[];
};
layout(std430, binding = 1) buffer OutputBuf {
vertex vertices_out[];
};

uniform mat3 projection;

void main() {
uint idx = gl_GlobalInvocationID.x;
//vertices_in[idx].position += vec4(0.5, 0, 0,1);
vertices_out[idx]=vertices_in[idx];

}

// void main() {
// uint idx = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y * gl_WorkGroupSize.x;
// // vertices_out[idx].pack[0].position = vec3(0);///* 0.1 * projection *  */vertices_in[idx].pack[0].position;
// // vertices_out[idx].pack[1].position = vec3(0);///* 0.1 * projection *  */vertices_in[idx].pack[1].position;
// // vertices_out[idx].pack[2].position = vec3(0);///* 0.1 * projection *  */vertices_in[idx].pack[2].position;
// // vertices_out[idx].pack[3].position = vec3(0);///* 0.1 * projection *  */vertices_in[idx].pack[3].position;
// // vertices_out[idx].position = vec3(0);///* 0.1 * projection *  */vertices_in[idx].pack[0].position;
// // vertices_out[idx].position = vec3(0);///* 0.1 * projection *  */vertices_in[idx].pack[1].position;
// // vertices_out[idx].position = vec3(0);///* 0.1 * projection *  */vertices_in[idx].pack[2].position;
// // vertices_out[idx].position = vec3(0);///* 0.1 * projection *  */vertices_in[idx].pack[3].position;

// vertices_out[idx].position = 10*vertices_out[idx].position;

// }