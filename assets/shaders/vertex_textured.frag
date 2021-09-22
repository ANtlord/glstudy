#version 330 core
in vec3 fragColor;
in vec2 texturePixel;

struct Material {
    sampler2D diffuseMap0; // GL_TEXTURE0
};

uniform Material material;

out vec4 Color;

void main()
{
    Color = texture(material.diffuseMap0, texturePixel);
}
