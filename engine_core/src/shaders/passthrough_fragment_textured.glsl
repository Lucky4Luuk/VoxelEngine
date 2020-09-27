in vec2 uv;

out vec3 frag_color;

uniform sampler2D renderedTexture;

void main() {
    frag_color = texture(renderedTexture, uv).xyz;
    // frag_color = vec3(uv, 0.0);
    // frag_color = vec3(1.0, 0.0, 0.0);
}
