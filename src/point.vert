#version 330 core

layout (location = 0) in vec3 Position;

uniform vec3 Offset;

void main() {
    gl_Position = vec4(Position + Offset, 1.0);
}
