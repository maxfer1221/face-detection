# face_detection

Multithreaded face detection in rust. 

Feature extraction done through [FAST](https://medium.com/data-breach/introduction-to-orb-oriented-fast-and-rotated-brief-4220e8ec40cf)

Libraries used:
 - [image](https://crates.io/crates/image): Image manipulation/creation
 - [crossbeam](https://crates.io/crates/crossbeam): Thread synchronization and scoping

TBD:
 - Quality/resolution options. Application currently samples 1/4 of the pixels for speed reasons.
 - Face detection.
