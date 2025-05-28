use audio::aggregate_device;

fn main() {
    let aggregate_device = aggregate_device::AudioAggregateDevice::builder("test-ag-de-name", "test-ag-de-uid")
        .private(false)
        .build();
    println!("aggregate_device: {aggregate_device:?}");
    println!("aggregate_device: {aggregate_device:?}");
}
