#version 330 core
in vec3 fragColor;
in vec2 texturePixel;

uniform sampler2D rubbish;

out vec4 Color;

void main()
{
    Color = texture(rubbish, texturePixel);
}
