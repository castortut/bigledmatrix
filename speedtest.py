import sys
import time

width = 72
height = 8
pos = width*height-1

while True:
    for column in range(width-1, -1, -1):
        tmp = 0
        for row in range(height):
            if pos == column * height + row:
                tmp |= 1 << row

        if tmp == ord('.'):
            # escape
            sys.stdout.buffer.write(tmp.to_bytes(1, 'big'))
        sys.stdout.buffer.write(tmp.to_bytes(1, 'big'))
    sys.stdout.buffer.write(b'.s')

    if pos == 0:
        pos = width*height - 1
    else:
        pos -= 1
    
    #time.sleep(0.001)


