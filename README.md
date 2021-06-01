

<div align="center">
  <img src="readme/anchor.png" width="120" height="auto" marginLeft="20px"/>

  <h1>atlas: a simple atlas packing tool</h1>

  Atlas takes a folder of images, and packs them into an atlas for use in game development. 
</div>

## Highlights
- **Easy to use**, even though it is a command line tool, it's simple and cross platform. 
- **Fast and safe** - written in Rust, atlas is lightning quick, with no unsafe code! 

## Features:

- Supports packing of textures of the same size.
- Allows specification of max number of columns, to maintain consistency as textures are added.
- Scales images to the desired width/height (In development).


## How to use it

Atlas takes a few arguments - specifically: 
- `-dir <dirname>` (required) where `dirname` is the name of the directory with the images to be bundled.  
- `-w <width>`, where `width` is the width of each image in the atlas (default 32px).
- `-h <height>`, where `height` is the height of each image in the atlas (default 32px).
- `-c <cols>`, where `cols` specifies the desired number of columns in the atlas (default to sqrt of # images). 

## Dependencies
- Uses [image](https://github.com/image-rs/image)


### References 

- <div>Icon made by <a href="https://www.freepik.com" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>
