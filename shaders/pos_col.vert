#version 150
in vec2 position;
in vec3 color;
uniform mat4 proj, view;
uniform vec2 center;        // part of model matrix
uniform float orientation;  // part of model martix

out vec3 frag_color;

void main()
{
    frag_color = color;
    float cosine = cos(orientation);
    float sine = sin(orientation);
    gl_Position = vec4(
            cosine * position.x - sine * position.y     + center.x,
            sine * position.x   + cosine * position.y   + center.y,
            0,
            1 );
	gl_Position = proj * view * gl_Position;
}
