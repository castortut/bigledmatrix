from PIL import Image
import sys

# Read a 72x8 image
image = Image.open(sys.argv[1])
width, height = image.size
assert height == 8

for column in range(width-1, -1, -1):

    # Build a single byte (display column) from 8 pixels:
    tmp = 0
    for row in range(height):
        pixel = image.getpixel((column, row))
        if pixel == 0:
            tmp |= 1 << row

    # escape the control-character by converting '.' -> '..'
    if tmp == ord('.'):
        sys.stdout.buffer.write(b'..')
    else:
        # Write the raw byte
        sys.stdout.buffer.write(tmp.to_bytes(1, 'big'))

# Strobe the shift register contents to display
sys.stdout.buffer.write(b'.s')


