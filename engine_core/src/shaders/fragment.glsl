struct Camera {
    vec3 position;
    vec3 settings;
};
uniform Camera camera;
uniform vec3 colour;

out vec3 frag_color;

void main() {
    frag_color = colour;
}
