# Chronicles - A crate for extracting various types of Archive files.

## Currently supporting the following formats:

- [Zip](https://www.info-zip.org/)
- [Tar](https://www.gnu.org/software/tar/)
- [Gz](https://www.gnu.org/software/gzip/)

## Todo -

- Support conversion between files.
- Support async operations.

## How to use -

```rust
use chronicles::extract;

let file = path::Path::new("someZip.zip");
      // Path to extract to
let to = path::Path::new("./extracted");
let res = extract(file, to);
```

## Supported extensions -

    - `.zip`
    - `.tar.gz`
    - `.tar.xz`
