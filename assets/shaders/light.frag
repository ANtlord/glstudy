#version 330 core
struct SpotLight {
    vec3 position;
    vec3 direction;
    vec3 ambient;
    float cutoff;
    float outerCutoff;
    // vec3 diffuse;
    // vec3 specular;
};

struct Material {
    sampler2D diffuseMap; // GL_TEXTURE0
    sampler2D specularMap; // GL_TEXTURE1
    sampler2D emissionMap; // GL_TEXTURE2
    float shininess;
};

struct PointLight {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 normal;
in vec3 fragPosition;
in vec2 texCoords;

out vec4 Color;

uniform vec3 viewPosition;
uniform PointLight light;
uniform SpotLight spotLight;
uniform Material material;

void main()
{
    vec3 fragNorm = normalize(normal);

    vec3 spotLightDir = normalize(spotLight.position - fragPosition);
    // Angle between:
    // - direction of the spot light source and
    // - direction from the source of the spot light to the current fragment.
    float theta = dot(spotLightDir, normalize(-spotLight.direction));
    vec3 ambientExtra = vec3(0);
    float normAgainstSpot = max(dot(normalize(-spotLight.direction), fragNorm), 0.0);
    // Greater because 0 <= theta <= 1. It's cos of angle NOT angle itself. If
    // 1 then spotLightDir and -spotLight.direction are the same
    if (theta > spotLight.cutoff) {
        ambientExtra = spotLight.ambient * normAgainstSpot;
    }

    // ambient
    vec3 ambient = light.ambient * vec3(texture(material.diffuseMap, texCoords));
    ambient = ambient + ambientExtra;

    // diffuse
    vec3 lightDir = normalize(light.position - fragPosition); // from light source to the fragment position?
    float diff = max(dot(fragNorm, lightDir), 0.0); // cos(angle between the vectors);
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuseMap, texCoords));

    // specular
    vec3 viewDir = normalize(viewPosition - fragPosition); // from view source to the fragment position?
    vec3 reflectDir = reflect(-lightDir, fragNorm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * vec3(texture(material.specularMap, texCoords));

    // emission
    vec3 emission = vec3(texture(material.emissionMap, texCoords));
    vec3 result = ambient + diffuse + specular + emission;
    Color = vec4(result, 1.0);
}
