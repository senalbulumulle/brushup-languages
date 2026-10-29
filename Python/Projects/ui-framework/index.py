import wgpu
from rendercanvas.auto import *
from pathlib import Path


## GLOBAL VARIABLES
TRIANGLE_SOURCE = "triangle.wgsl"                           ## Triangle Source

## initializing the device adapter
adapter = wgpu.gpu.request_adapter_sync()
print(adapter.info)

## initializing the device gpu
device = adapter.request_device_sync()


## initializing the canvas window or display
canvas = RenderCanvas(size=(640, 480), title="ui-framework")

context = canvas.get_wgpu_context()
texture_format = context.get_preferred_format(device.adapter)
context.configure(device=device, format=texture_format)


## include the triangle shader file from the global variable on the top 
shadertriangle = (Path(__file__).parent / TRIANGLE_SOURCE).read_text(encoding="utf-8")