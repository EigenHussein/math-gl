#version 460 core

//#define COLOR_TEXTURE

in vec3 f_position;
in vec3 f_normal;
in vec4 f_color;
in vec2 f_txcoord;

out vec4 Color;

#if defined(COLOR_TEXTURE)
uniform sampler2D TX;
#endif

void main() {
    #ifndef COLOR_TEXTURE
    Color = f_color; //vec4(1);
    #else
    Color = vec4(texture2D(TX, f_txcoord).rgb, 1);
    #endif
}