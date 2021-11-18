# face_detection

### Multithreaded face detection in rust. 

## Usage
1. `git clone https://github.com/maxfer1221/face_detection`
2. `cd face_detection`
3. `cargo run src/my_image.jpg 5 1`, `cargo run path/to/impage feature_threshold thread_count`

## More information
Feature extraction done through [FAST](https://medium.com/data-breach/introduction-to-orb-oriented-fast-and-rotated-brief-4220e8ec40cf)

### Libraries used:
 - [image](https://crates.io/crates/image): Image manipulation/creation
 - [crossbeam](https://crates.io/crates/crossbeam): Thread synchronization and scoping

### TBD:
 - Quality/resolution options. Application currently samples 1/4 of the pixels for speed reasons.
 - Face detection.
 - Video face tracking
 - Camera implementation
