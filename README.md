# glot-images


## Overview
glot-images are the docker images used by [glot.io](https://glot.io) to run code.
The images are built using nix which are pinned to a specific nixpkgs commit to create reproducible images.
See the [overview](https://github.com/glotcode/glot) on how everything is connected.

#### Code runner
The [code-runner](https://github.com/glotcode/code-runner) is installed in each image which writes the files inside the container, compiles and runs the code and returns the result as a json payload.

#### Images
Prebuilt images can be found on [Docker Hub](https://hub.docker.com/u/glot).
(If nix is unfamiliar you can find some old examples using Dockerfile [here](https://github.com/prasmussen/glot-containers))


## Example

```bash
echo '{
  "language": "javascript",
  "files": [{
    "name": "main.js",
    "content": "console.log(\"Hello World!\");"
  }]
}' | docker run -i --rm glot/javascript:latest
```

##### Result
```json
{"stdout":"Hello World!\n","stderr":"","error":""}
```
