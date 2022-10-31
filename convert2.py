from PIL import Image
import sys
import time

def print8(image_region, ymin, ymax):
    width, height = image_region.size

    assert ymax - ymin == 7
    assert 0 <= ymin <= height
    assert 0 <= ymax <= height

    for column in range(width-1, -1, -1):

        # Build a single byte (display column) from 8 pixels:
        tmp = 0
        for row in range(height):
            if ymin <= row <= ymax:
                pixel = image.getpixel((column, row))
                if pixel in [ 0, (0,0,0), (0,0,0,255)]:
                    tmp |= 1 << (row - ymin)

        # escape the control-character by converting '.' -> '..'
        if tmp == ord('.'):
            sys.stdout.buffer.write(b'..')
        else:
            # Write the raw byte
            sys.stdout.buffer.write(tmp.to_bytes(1, 'big'))

    # Strobe the shift register contents to display
    sys.stdout.buffer.write(b'.s')

def display(image):
    if height == 8:
        print8(image, 0, 7)
    elif height == 16:
        sys.stdout.buffer.write(b'.0')
        print8(image, 0, 7)
        sys.stdout.buffer.write(b'.1')
        print8(image, 8, 15)


# Read a 72x8 or 72x16 image
image = Image.open(sys.argv[1])
width, height = image.size

if image.is_animated:
    for frame in range(image.n_frames):
        image.seek(frame)
        display(image)
        time.sleep(0.1)
else:
    display(image)
