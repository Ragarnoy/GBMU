#version 330
in vec2 text_coord;
uniform sampler2D render_texture;
out vec4 out_color;
void main(){
  out_color=texture(render_texture,text_coord*vec2(1.,-1.));
}
