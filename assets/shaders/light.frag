#version 330 core

in vec3 normal;
in vec3 fragpos;

out vec4 Color;

uniform vec3 lightPosition;
uniform vec3 lightColor;
uniform vec3 objectColor;

void main()
{
    vec3 lightDir = normalize(lightPosition - fragpos);
    float diff = max(dot(lightDir, normal), 0.0); // cos(angle between the vectors);
    vec3 diffuse = lightColor * diff;

    float ambientStrength = 0.1;
    vec3 ambient = lightColor * ambientStrength;
    Color = vec4((diffuse + ambient) * objectColor, 1.0);
}
