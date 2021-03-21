
## Tasks:
- 1.read cube-face image(s);
	> Output: CubeState (6-Faces object);
- 2.solve method;
	> Output: Array of FaceRotation,


## Refers:
https://github.com/Wiston999/python-rubik



===================
cargo test -- --nocapture

```$rust
// let aaa: HashMap<&str, Vec<&str>> = [
//     ("U", vec!["x"])
// ].iter().cloned().collect();
// let steps: HashMap<&str, HashMap<&str, Vec<&str>>> = HashMap::new();
// steps.insert("U", aaa)

let aaa: HashMap<String, Vec<String>> = [
    ("U".into(), ["x"].iter().map(|&x| x.into()).collect())
].iter().cloned().collect();
return aaa;
```

+
