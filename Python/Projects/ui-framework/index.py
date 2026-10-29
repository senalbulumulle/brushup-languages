import wgpu
from rendercanvas.auto import *

## initializing the device adapter
adapter = wgpu.gpu.request_adapter_sync()
print(adapter.info)

## initializing the device gpu
device = adapter.request_device_sync()


## initializing the canvas window or display
canvas = RenderCanvas(size=(640, 480), title="ui-framework")