//Vertex semantics
in vec3 position;

//Shader interface
uniform mat4 projection;
uniform mat4 view;
// uniform mat4 model;

void main() {
    gl_Position = projection * view * vec4(position, 1.0);
}
