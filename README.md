# Melodia
This project is meant to be a complete rewrite of the work I have done so far in Python & NodeJS for a project called [Melodia](https://gagikamaryan.com/melodya/), where I am turned an old Soviet radio into a smart speaker. The older project is now abandoned as I am focusing on this Rust version but you can view it [here](https://github.com/gagik/melodia-python-node).

[![Melodia Showcase](https://i.imgur.com/9qGICXv.png)](https://vimeo.com/457722159 "Watch Melodia Showcase!")

### Motivation behind rewriting this project in Rust
1. **Efficiency**: I started realizing that what was originally meant to be Python with some elements of Node was turning into a very messy and probably really slow code. Rust bindings to the LED matrix library work much, *much* faster and it is easier to access system elements like sound thanks to the variety of C library ports.
2. **To challenge myself**: The code I was writing was turning *too familiar* in a way where it was boring and unrewarding. I had never programmed in Rust yet have always had great interest in it so this seemed like a great opportunity to learn and get out of my comfort zone a bit.
3. **Cleaner, more logical code**: With higher level languages, it is very easy to get carried away and write programs which work "well enough" yet could have been implemented much, much better. What I love about Rust is that it forces you to think about how your code works.


## Current Features
- Volume control through knobs and volume visualizer on the LED
- Fun graphic examples (to be included in

## TODO List
To be ported from the Node.js / Python version:
- GIF visualization
- More basic graphics library functions

To be added:
- Spotify cover art visaulization
- Program selection UI
- "Extension" / "App" API
