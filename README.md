# Canved

![Screenshot of brush mode](https://user-images.githubusercontent.com/8389938/125021660-1a18cf00-e049-11eb-8a97-c1ecdc68850d.png)

A lightweight and minimalistic image editor for Unix systems\*. 

<sub>\*: although it should work on any platform</sub>

## Features

- [X] Supports PNG, JPEG, GIF, ICO, TGA and BMP for image input and output.
- [X] Supports standard input and output.
- [X] Undo/redo any modifications.
- [X] Mode selection.
	- [X] View mode.
	- [X] Brush mode. Select a color with the number keys and paint with the mouse. Resize brush with scrollwheel.
		- [ ] Inverted color brush
	- [X] Crop mode. Select an area of the image to crop.
	- [ ] Text mode. Write text in the desired size.
	- [ ] Selection mode. Select a portion of the image and move it around, resize it, transform it in general. Also allow copying and pasting from the clipboard.
	- [ ] Rotation mode. Rotate and flip the image.
	- [ ] Color picking mode. 
- [ ] Configuration file 
	- [ ] Color palette customization.
	- [ ] Defaults: brush size, starting mode, etc.
	- [ ] Font selection.
	- [ ] Customizable key bindings.
	- [ ] Limit version buffer (undo history) size.
	- [ ] Optional Xresources support
- [ ] Change backend to wgpu/miniquad/etc. (minifb is awesome but a bit limiting).
	- [ ] Draw all UI using GPU acceleration, instead of canvas buffers.
- [ ] Support all other `image` crate formats (Pnm, Farbfeld, etc).
- [ ] Less memory footprint for versioning (undo history). Will probably use a delta-based versioning system.

## Example use cases

Take a screenshot, edit it, then output to the clipboard, with the use of [`shotgun`](https://github.com/neXromancers/shotgun).

```shell
shotgun - | canved - -o - | xclip -t image/png -selection
```

Edit a file, then output it as a JPEG.

```shell
canved image.png -o edited.jpg
```

## Shell usage

See the usage with `canved --help`.

## Editor usage

Once editing an image, the following keybinds are in action:

- Q: Save and quit.
- Ctrl+Z: Undo.
- Ctrl+Shift+Z: Redo.

The editor's state is a *mode*. You can switch between modes with keys:

- Escape: View mode/normal mode.
- B: Brush mode.
- C: Crop mode.

### View/normal mode

See the image, hiding all the other UI.

### Brush mode

Paint with the mouse, select the color with the keyboard numbers 1-9.

### Crop mode

Select an area to crop with the mouse.

## Origin

I got the idea for this project in October 2020, because I couldn't find an image editor that would simply allow me to add arrows, circles or text to a screenshot I'd taken, without taking 15+ seconds to load on my HDD. Not only that, but most image editors I'd found had too big of a scope for what I was trying to do (Krita, GIMP, I'd even say Pinta). I do like all of them, but when doing more complex operations, like collages or texture edition for gamedev. 

And thus, `canved` was born in May 2021, when I finally figured out a way to do it using `minifb`. The scope of this project is not that big. Once the features shown above are complete, there will probably only be bugfixes and optimizations, unless an useful enough feature is requested/contributed.

Other goals of canved.

- To load and save as quickly as possible.
- To be as lightweight as possible, in terms of dependencies and installation.
