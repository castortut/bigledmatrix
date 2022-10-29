import sys
import time

patterns = [
    0b11111111,
    0b01111111,
    0b00111111,
    0b00011111,
    0b00001111,
    0b00000111,
    0b00000011,
    0b00000001,
]

sys.stdout.buffer.write(b'.q.c.s')
while True:
    for pattern in patterns:
        #sys.stdout.buffer.write(b'.c')
        for i in range(bin(0xFF - pattern).count('1') ** 3):
            sys.stdout.buffer.write(pattern.to_bytes(1, 'big'))
            sys.stdout.buffer.write(b'.s')


