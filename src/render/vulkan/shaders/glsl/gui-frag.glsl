#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (binding = 0) uniform sampler2D tex;

layout (location = 0) in vec4 texcoord;

layout (location = 0) out vec4 frag_color;

void main() {
	frag_color = texture(tex, texcoord.xy);
}
