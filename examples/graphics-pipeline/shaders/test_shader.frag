#version 450

layout( location = 0 ) out vec4 outColor;

layout( location = 0 ) in vec2 inUV;

void main()
{
    outColor = vec4( inUV, 1.0f, 1.0f );
}