#version 330 core

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;

out vec3 normal;
out vec3 fragpos;

uniform mat4 model; // local -> world space
uniform mat4 view; // world -> view space
uniform mat4 projection; // view -> clip space

void main()
{
    gl_Position = projection * view * model * vec4(inPosition, 1.0);
    fragpos = vec3(model * vec4(inPosition, 1.0));
    normal = mat3(transpose(inverse(model))) * inNormal;
}
