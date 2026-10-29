import wgpu


adapter = wgpu.gpu.request_adapter_sync()
print(adapter.info)
