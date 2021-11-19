# face_detection

### Multithreaded face detection in rust. 

## Usage
1. `git clone https://github.com/maxfer1221/face_detection`
2. `cd face_detection`
3. `cargo run in/example1.jpg 5 1 3`, `cargo run path/to/image feature_threshold thread_count step_size`

`feature_threshold`: Dictates feature sensitivity. Higher values means less, more striking features will be found.

`thread_count`: Dictates how many threads the program can spawn. Minimum of 1.

`step_size`: Dictates how many pixels the program samples. A step size of 1 would sample every pixel, a step size of 2 would sample 1/4th of the pixels (half the width, half the height), etc.

### Current Feature Extraction [Examples](https://github.com/maxfer1221/face_detection/tree/main/out)
<p float="left">
 <img display="inline" src="https://github.com/maxfer1221/face_detection/blob/main/in/example1.jpg?raw=true" alt="example_1" height="320">
 <img src="https://github.com/maxfer1221/face_detection/blob/main/out/example1.png?raw=true" alt="example_1_out" height="320">
</p>
<p float="left">
 <img display="inline" src="https://github.com/maxfer1221/face_detection/blob/main/in/example2.jpeg?raw=true" alt="example_2" width="400">
 <img src="https://github.com/maxfer1221/face_detection/blob/main/out/example2.png?raw=true" alt="example_2_out" width="400">
</p>
<p float="left">
 <img display="inline" src="https://github.com/maxfer1221/face_detection/blob/main/in/example3.jpg?raw=true" alt="example_3" width="400">
 <img src="https://github.com/maxfer1221/face_detection/blob/main/out/example3.png?raw=true" alt="example_3_out" width="400">
</p>
Created with a `feature_threshold` of 6 and a `step_size` of 1

## More information
Feature extraction done through [FAST](https://medium.com/data-breach/introduction-to-orb-oriented-fast-and-rotated-brief-4220e8ec40cf)

### Libraries used
 - [image](https://crates.io/crates/image): Image manipulation/creation
 - [crossbeam](https://crates.io/crates/crossbeam): Thread synchronization and scoping

### TBD
 - ~~Quality/resolution options. Application currently samples 1/4 of the pixels for speed reasons~~
 - Face detection
 - Video face tracking
 - Camera implementation
