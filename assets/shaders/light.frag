#version 330 core
struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 normal;
in vec3 fragPosition;

out vec4 Color;

uniform vec3 viewPosition;
uniform Light light;
uniform Material material;

void main()
{
    // ambient
    vec3 ambient = light.ambient * material.ambient;

    // diffuse
    vec3 fragNorm = normalize(normal);
    vec3 lightDir = normalize(light.position - fragPosition); // from light source to the fragment position?
    float diff = max(dot(fragNorm, lightDir), 0.0); // cos(angle between the vectors);
    vec3 diffuse = light.diffuse * diff * material.diffuse;

    // specular
    vec3 viewDir = normalize(viewPosition - fragPosition); // from view source to the fragment position?
    vec3 reflectDir = reflect(-lightDir, fragNorm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * material.specular;

    vec3 result = (ambient + diffuse + specular);
    Color = vec4(result, 1.0);
}
