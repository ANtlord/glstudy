#version 330 core

in vec3 normal;
in vec3 fragPosition;

out vec4 Color;

uniform vec3 lightPosition;
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform vec3 viewPosition;

void main()
{
    // ambient
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // diffuse
    vec3 fragNorm = normalize(normal);
    vec3 lightDir = normalize(lightPosition - fragPosition); // from light source to the fragment position?
    float diff = max(dot(fragNorm, lightDir), 0.0); // cos(angle between the vectors);
    vec3 diffuse = diff * lightColor;

    // specular
    float specularStrength = 0.5;
    vec3 viewDir = normalize(viewPosition - fragPosition); // from view source to the fragment position?
    vec3 reflectDir = reflect(-lightDir, fragNorm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * objectColor;
    Color = vec4(result, 1.0);
}
