
/*
#define MODEL_MATRIX
#define COLOR_SINGLE
#define COLOR_PER_POSITION
#define COLOR_PER_VERTEX
#define COLOR_TEXTURE
*/

// #version 460 core
// #define COLOR_PER_POSITION

layout(location = 0) in vec4 v_position;
layout(location = 1) in vec4 v_color;
layout(location = 2) in vec3 v_normal;
layout(location = 3) in vec2 v_txcoord;

uniform mat4 Proj;
uniform mat4 View;
uniform mat4 Model;
uniform vec4 solid_color = vec4(1.0);

out vec3 f_position;
out vec3 f_normal;
out vec4 f_color;
out vec2 f_txcoord;

layout(location = 2) uniform uint shape;

// layout(std430, binding = 0) buffer Buf1 {
// mat4 SphereInstance_Translation[];
// };
// layout(std430, binding = 1) buffer Buf2 {
// mat4 CylinderInstance_Translation[];
// };

layout(std430, binding = 0) buffer instance_buf {
mat4 InstanceModel[];
};

void main() {

vec4 v_pos = Proj * View * vec4(v_position.xyz, 1);
    #if defined(MODEL_MATRIX)
v_pos = Proj * View * Model * vec4(v_position.xyz, 1);
    #endif

    #if defined(INSTANCED)
v_pos = Proj * View * InstanceModel[gl_InstanceID + shape] * vec4(v_position.xyz, 1);
    #endif

gl_Position = v_pos;
    #if defined(COLOR_SINGLE)
f_color = solid_color;
    #endif

    #if defined(COLOR_PER_POSITION)
f_color = vec4(0.2 + 0.8 * v_position.xyz, 1);
    #endif

    #if defined(COLOR_PER_VERTEX)
f_color = v_color;
    #endif
    #if defined(COLOR_TEXTURE) || defined(COLOR_BLOOM)
f_color = solid_color;
f_txcoord = v_txcoord;
    #endif

f_position = v_pos.xyz;
f_normal = v_normal;
}