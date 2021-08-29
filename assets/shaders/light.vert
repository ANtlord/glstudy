#version 330 core

layout (location = 0) in vec3 inPosition;
layout (location = 1) in vec3 inNormal;
layout (location = 2) in vec2 inTexCoords;

out vec3 normal;
out vec3 fragPosition;
out vec2 texCoords;

uniform mat4 model; // local -> world space
uniform mat4 view; // world -> view space
uniform mat4 projection; // view -> clip space

void main()
{
    fragPosition = vec3(model * vec4(inPosition, 1.0));
    normal = mat3(transpose(inverse(model))) * inNormal;
    texCoords = inTexCoords;
    gl_Position = projection * view * vec4(fragPosition, 1.0);
}
