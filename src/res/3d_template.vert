// wengwengweng

attribute vec3 pos;
attribute vec3 normal;
attribute vec4 color;

// varying vec2 v_uv;
varying vec4 v_color;
varying float v_brightness;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;
uniform vec3 light_pos;
uniform float time;

###REPLACE###

void main() {

	vec3 world_pos = (model * vec4(pos, 1.0)).xyz;
	vec3 unit_normal = normalize((model * vec4(normal, 1.0)).xyz);
	vec3 unit_light_dir = normalize(light_pos - world_pos);

	v_brightness = max(dot(unit_normal, unit_light_dir), 0.0);
	v_color = color;
// 	v_uv = uv;

	gl_Position = proj * view * vec4(world_pos, 1.0);

}

