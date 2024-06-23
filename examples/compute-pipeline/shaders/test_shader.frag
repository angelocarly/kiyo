#version 450

layout( location = 0 ) out vec4 outColor;

layout( location = 0 ) in vec2 inUV;

layout ( binding = 0, rgba8 ) readonly uniform image2D inImage;

void main()
{
    ivec2 screenSize = imageSize( inImage );
    vec4 color = imageLoad( inImage, ivec2( inUV * screenSize ) );
    outColor = color;
}