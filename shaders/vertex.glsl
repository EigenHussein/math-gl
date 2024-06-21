
/*
#define MODEL_MATRIX
#define COLOR_SINGLE
#define COLOR_PER_POSITION
#define COLOR_PER_VERTEX
#define COLOR_TEXTURE
*/


#version 460 core
#define COLOR_PER_POSITION

layout(location = 0) in vec3 v_position;
layout(location = 1) in vec4 v_color;
layout(location = 2) in vec3 v_normal;
layout(location = 3) in vec2 v_txcoord;

uniform mat4 Proj;
uniform mat4 View;
uniform mat4 Model;
uniform vec4 solid_color;

out vec3 f_position;
out vec3 f_normal;
out vec4 f_color;
out vec2 f_txcoord;

void main() {
    #if defined(MODEL_MATRIX)
    vec4 v_pos = Proj * View * Model * vec4(v_position.xyz, 1);
    #else
    vec4 v_pos = Proj * View * vec4(v_position.xyz, 1);
    #endif

    gl_Position = v_pos;

    #if defined(COLOR_SINGLE)
    f_color = solid_color;
    #endif

    #if defined(COLOR_PER_POSITION)
    f_color = vec4(v_position, 1);
    #endif

    #if defined(COLOR_PER_VERTEX)
    f_color = v_color;
    #endif
    #if defined(COLOR_TEXTURE)
    f_color = solid_color;
    f_txcoord = v_txcoord;
    #endif

}