#version 330
uniform vec2 scale;
uniform vec2 offset;
in vec2 position;
out vec2 text_coord;
void main(){
	text_coord=(position+1.)*.5;
	gl_Position=vec4(position*scale+offset,0.,1.);
}
