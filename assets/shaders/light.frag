#version 330 core
struct SpotLight {
    vec3 position;
    vec3 direction;
    vec3 ambient;
    float cutoff;
    float outerCutoff;
    vec3 diffuse;
    vec3 specular;
};

struct Material {
    sampler2D diffuseMap; // GL_TEXTURE0
    sampler2D specularMap; // GL_TEXTURE1
    sampler2D emissionMap; // GL_TEXTURE2
    float shininess;
};

struct DirectionalLight {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
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
uniform DirectionalLight directionalLight;
uniform Material material;

float getIntensity(float theta, float outerCutoff, float cutoff) {
    float numenator = theta - outerCutoff;
    float epsilon = cutoff - outerCutoff;
    float ret = clamp(numenator / epsilon, 0., 1.);
    return ret;
}

vec3 computePointLight() {
    vec3 fragNorm = normalize(normal);
    // ambient
    vec3 ambient = light.ambient * vec3(texture(material.diffuseMap, texCoords));

    // diffuse
    vec3 lightDir = normalize(light.position - fragPosition); // from light source to the fragment position?
    float diff = max(dot(fragNorm, lightDir), 0.0); // cos(angle between the vectors);
    vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuseMap, texCoords));

    // specular
    vec3 viewDir = normalize(viewPosition - fragPosition); // from view source to the fragment position?
    vec3 reflectDir = reflect(-lightDir, fragNorm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * spec * vec3(texture(material.specularMap, texCoords));
    return ambient + diffuse + specular;
}

vec3 computeSpotLight(float intensity) {
    vec3 fragNorm = normalize(normal);
    // ambient
    vec3 ambient = spotLight.ambient * vec3(texture(material.diffuseMap, texCoords));

    // diffuse
    vec3 lightDir = normalize(spotLight.position - fragPosition); // from spotLight source to the fragment position?
    float diff = max(dot(fragNorm, lightDir), 0.0); // cos(angle between the vectors);
    vec3 diffuse = spotLight.diffuse * diff * vec3(texture(material.diffuseMap, texCoords));

    // specular
    vec3 viewDir = normalize(viewPosition - fragPosition); // from view source to the fragment position?
    vec3 reflectDir = reflect(-lightDir, fragNorm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = spotLight.specular * spec * vec3(texture(material.specularMap, texCoords));

    specular *= intensity;
    diffuse *= intensity;
    return ambient + diffuse + specular;
}

vec3 computeDirectionalLight() {
    vec3 fragNorm = normalize(normal);
    // ambient
    vec3 ambient = directionalLight.ambient * vec3(texture(material.diffuseMap, texCoords));

    // diffuse
    vec3 lightDir = normalize(directionalLight.direction);
    float diff = max(dot(fragNorm, lightDir), 0.0); // cos(angle between the vectors);
    vec3 diffuse = directionalLight.diffuse * diff * vec3(texture(material.diffuseMap, texCoords));

    // specular
    vec3 viewDir = normalize(viewPosition - fragPosition); // from view source to the fragment position?
    vec3 reflectDir = reflect(-lightDir, fragNorm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = directionalLight.specular * spec * vec3(texture(material.specularMap, texCoords));

    return ambient + diffuse + specular;
}

void main() {
    // vec3 emission = vec3(texture(material.emissionMap, texCoords));

    vec3 fragNorm = normalize(normal);

    vec3 spotLightDir = normalize(fragPosition - spotLight.position);
    float theta = dot(spotLightDir, normalize(spotLight.direction));
    float intensity = getIntensity(theta, spotLight.outerCutoff, spotLight.cutoff);

    vec3 spotlight = computeSpotLight(intensity);
    vec3 pointlight = computePointLight();
    vec3 directionallight = computeDirectionalLight();
    vec3 result = spotlight + pointlight + directionallight;// + emission;
    Color = vec4(result, 1.0);
}
