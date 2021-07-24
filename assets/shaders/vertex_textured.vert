#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 texturePosition;

uniform mat4 model; // local -> world space
uniform mat4 view; // world -> view space
uniform mat4 projection; // view -> clip space

out vec3 fragColor;
out vec2 texturePixel;

void main()
{
    gl_Position = projection * view * model * vec4(position, 1.0);
    fragColor = color;
    texturePixel = texturePosition;
}
