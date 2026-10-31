import wgpu
import os
import json
import sys

from rendercanvas.auto import RenderCanvas, loop



wgsl_shader_source = """
struct VertexOutput {
    @location(0) color : vec4f,
    @builtin(position) pos: vec4f,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var positions = array<vec2f, 3>(
        vec2(0.0, -0.5),
        vec2(0.5, 0.5),
        vec2(-0.5, 0.75),
    );
    // vertex attributes are interpolated in the fragment shader.
    var colors = array<vec3f, 3>(  // srgb colors
        vec3(1.0, 1.0, 0.0),
        vec3(1.0, 0.0, 1.0),
        vec3(0.0, 1.0, 1.0),
    );
    var out: VertexOutput;
    out.pos = vec4(positions[index], 0.0, 1.0);
    out.color = vec4(colors[index], 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let physical_color = pow(in.color.rgb, vec3(2.2));  // gamma correct
    return vec4(physical_color, in.color.a);
}
"""



adapter = wgpu.gpu.request_adapter_sync(power_preference="high-performance")
device = adapter.request_device_sync()


canvas = RenderCanvas(size=(640, 480), title="ui framework")
context = canvas.get_wgpu_context()
render_texture_format = context.get_preferred_format(device.adapter)
context.configure(device=device, format=render_texture_format)


shader = device.create_shader_module(code=wgsl_shader_source)


'''
@docs
We might have to move the render_pipeline to a separate python file for 
reusablity, since it is basically a reusablity config for wgpu to 
render the display. Will make it completely organized that way

'''

render_pipeline = device.create_render_pipeline(
        **wgpu.RenderPipelineDescriptor(
            layout=wgpu.AutoLayoutMode.auto,
            vertex=wgpu.VertexState(module=shader),
            depth_stencil=None,
            multisample=None,
            fragment=wgpu.FragmentState(
                module=shader,
                targets=[wgpu.ColorTargetState(format=render_texture_format)],
                ),
            )
        )



def drawing_function():
    command_encoder = device.create_command_encoder()
    current_texture_view = context.get_current_texture().create_view()

    render_pass = command_encoder.begin_render_pass(
            color_attachments=[
                wgpu.RenderPassColorAttachment(
                    view=current_texture_view,
                    clear_value=(0, 0.2, 0, 1),  # a green background
                    load_op=wgpu.LoadOp.clear,
                    store_op=wgpu.StoreOp.store,
                    )
                ],
            )
    render_pass.set_pipeline(render_pipeline)
    render_pass.draw(3)
    render_pass.end()
    device.queue.submit([command_encoder.finish()])



'''
@docs
call the functions using the canvas.request_draw(). I need to figure
out to make an array with other functions for the request_draw() but
let's see
'''
canvas.request_draw(drawing_function)


if __name__ == "__main__":
    loop.run()
