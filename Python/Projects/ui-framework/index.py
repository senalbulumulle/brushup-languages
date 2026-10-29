import wgpu

## initializing the device adapter
adapter = wgpu.gpu.request_adapter_sync()
print(adapter.info)

## initializing the device gpu
device = adapter.request_device_sync()