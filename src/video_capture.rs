use rscam;
use std::fs;
use std::io::Write;

pub fn capture_video(dur: (bool, usize), fps: (u32, u32), res: (u32, u32)) {
    let mut camera = rscam::new("/dev/video0").unwrap();

    camera.start(&rscam::Config {
        interval: fps,
        resolution: res,
        format: b"MJPG",
        ..Default::default()
    }).unwrap();

    for i in 0..dur.1 {
        let frame = camera.capture().unwrap();
        let mut file = fs::File::create(&format!("frames/frame-{}.jpg", i)).unwrap();
        file.write_all(&frame[..]).unwrap();
    }
}
