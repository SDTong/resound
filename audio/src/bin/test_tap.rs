use audio::process;
use audio::tap;

fn main() {
    let processes = process::list();
    assert!(processes.is_ok(), "process: {processes:?}");
    let mut processes = processes.unwrap();
    assert!(!processes.is_empty(), "no have process");
    let process = processes.pop().unwrap();

    for _ in 0..1 {
        let tap_builder = tap::AudioTapDescriptionBuilder {
            name: "audio-tap".to_string(),
            uid: None,
            processes: vec![process.get_id()],
            mono: false,
            exclusive: false,
            mixdown: true,
            private: false,
            device_uid: None,
            stream: None,
        };
        let tap = tap_builder.build();
        println!("tap: {:?}", tap);
        assert!(tap.is_ok());
    }
}
