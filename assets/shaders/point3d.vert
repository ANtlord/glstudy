#version 330 core

layout (location = 0) in vec3 position;

uniform mat4 model; // local -> world space
uniform mat4 view; // world -> view space
uniform mat4 projection; // view -> clip space

void main()
{
    gl_Position = projection * view * model * vec4(position, 1.0);
}
