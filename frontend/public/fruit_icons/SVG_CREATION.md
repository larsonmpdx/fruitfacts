my process for making these fruit SVGs:

- find a source image that will translate nicely to vector graphics (DALL-E 2 notes below - a good way to get sources). most of the resulting quality of the vector output is from the source itself. it should have high resolution, nice contrast for the details wanted, simplicity, etc.
- open the source image in paint dot net (bitmap editor) and trim it down to the thing you want on a white background with fill or paintbrush or whatever
  - remember to change the pen mode to "overwrite"
- save as png
- open png in inkscape (vector editor)
  - open bitmap
  - path->trace bitmap
    - single scan, autotrace, OR colors. 4 filter iterations, 2 error threshold
    - speckles 120
    - smooth 1.34
    - optimize 5
    - select remove background
    - play with the trace settings, this is the 2nd major quality filter
  - make sure the imported bitmap is selected then click update
  - sometimes this fails or other settings work better. click apply to create the vectors on the canvas
  - delete underlying image (from objects view) and use path->simplify once or multiple times to reduce size and simplify
  - "edit nodes" (F2) can select parts of the path for manual simplification (many times there are background colors with details that are beneath other colors that can all be deleted)
  - resize canvas: ctrl-shft-d (document properties) -> click "resize to content" button
  - save as plain svg
  - upload to an svg minifier (svgomg gui) and save the output
  - manually edit in the svg's "width" and "height" attributes like the other SVGs. the viewbox will still keep the right aspect ratio
  - the resulting image should be small and have a transparent background (no background)

# DALL-E 2 prompts for source images
- sign up at https://labs.openai.com/
- see book: http://dallery.gallery/wp-content/uploads/2022/07/The-DALL%C2%B7E-2-prompt-book.pdf
- `quince fruit, no leaves, vector graphics, white background`
  - this gave nice images but cropped. download and zoom out using paint.net or something, leaving the exterior transparent. upload and generate. remove at least one pixel from the original image. give it the same prompt as before
- `olive branch with olives, vector graphics, white background` gave a nice result
