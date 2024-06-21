// #version 460 core

//#define COLOR_TEXTURE

in vec3 f_position;
in vec3 f_normal;
in vec4 f_color;
in vec2 f_txcoord;

out vec4 Color;

//#if defined(COLOR_TEXTURE) || defined(COLOR_BLOOM)
uniform sampler2D TX;
//#endif

// void main(){
//     Color = vec4(1);
// }

void main() {

    const vec3 relative_light_position = normalize(vec3(50, 50, 50) - f_position);
    float light_intensity = max(0.2, dot(f_normal, relative_light_position));
#ifndef COLOR_TEXTURE
    Color = vec4(f_color.xyz * light_intensity, 1);//f_color; //vec4(1);
#endif

#if defined(COLOR_TEXTURE)
    float red_val = texture(TX, f_txcoord).r;
    Color = vec4(1 - red_val);//1-vec4(texture(TX, f_txcoord).rgba);//vec4(vec3(1-red_val),1-red_val);//vec4(texture(TX, f_txcoord).r + vec3(0.2), 1);
#endif

#if defined(COLOR_BLOOM)
    vec3 res = vec3(0);
    for(int i = 0; i < 7; i++) {

        res += texture(TX, f_txcoord, i).rgb;

    }
    Color = vec4(texture(TX, f_txcoord).rgb + res / 3.0, 1);
#endif

#if defined(COLOR_PER_VERTEX)
    Color = f_color;

#endif

}