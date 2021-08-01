#version 330 core

in vec3 normal;
in vec3 fragpos;

out vec4 Color;

uniform vec3 lightPosition;
uniform vec3 lightColor;
uniform vec3 objectColor;

void main()
{
    // diffuse
    vec3 norm = normalize(normal);
    vec3 lightDir = normalize(lightPosition - fragpos);
    float diff = max(dot(lightDir, norm), 0.0); // cos(angle between the vectors);
    vec3 diffuse =  diff * lightColor;

    // ambient
    float ambientStrength = 0.15;
    vec3 ambient = lightColor * ambientStrength;

    Color = vec4((ambient + diffuse) * objectColor, 1.0);
}
