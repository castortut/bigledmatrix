from PIL import Image
import sys

image = Image.open(sys.argv[1])
width, height = image.size
print(image.size)
assert height == 8

pixels = []

for column in range(width):
    for row in range(height):
        pixel = image.getpixel((column, row))
        pixels.append(pixel)

print(pixels[::-1])


