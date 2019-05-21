### Getting Started

An example Dockerfile is included here to install Rust on an Ubuntu 18.04 Image

Something like the following should build the image:

```
docker build -t $(whoami):install_rust -f Dockerfile.install_rust .
```

Once the image is built, we can then run the container and see the output of the version installed:

```
docker run --rm $(whoami):install_rust      
rustc 1.34.2 (6c2484dc3 2019-05-13)
```

While this is great for testing purposes, I currently enjoy using `homebrew` so will be installing it that way

 - https://formulae.brew.sh/formula/rust

 
