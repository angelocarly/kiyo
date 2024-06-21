#version 450

layout( location = 0 ) out vec4 outColor;

layout( location = 0 ) in vec2 inUV;

layout ( binding = 0, rgba8 ) readonly uniform image2D inImage;

void main()
{
    vec4 color = imageLoad( inImage, ivec2( inUV * vec2( 800, 600 ) ) );
    outColor = color;
}