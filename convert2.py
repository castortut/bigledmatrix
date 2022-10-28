from PIL import Image
import sys

image = Image.open(sys.argv[1])
width, height = image.size
assert height == 8

pixels = []

for column in range(width-1, -1, -1):
    tmp = 0
    for row in range(height):
        pixel = image.getpixel((column, row))
        if pixel == 0:
            tmp |= 1 << row

    #print(bin(tmp))
    if tmp == ord('.'):
        # escape
        sys.stdout.buffer.write(tmp.to_bytes(1, 'big'))
    sys.stdout.buffer.write(tmp.to_bytes(1, 'big'))
        

sys.stdout.buffer.write(b'.s')


