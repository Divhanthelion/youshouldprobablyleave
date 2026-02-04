import struct
import os

# Create minimal 16x16 ICO file
header = struct.pack('<HHH', 0, 1, 1)
bmp_size = 40 + 16*16*4
entry = struct.pack('<BBBBHHII', 16, 16, 0, 0, 1, 32, bmp_size, 22)
dib = struct.pack('<IiiHHIIiiII', 40, 16, 32, 1, 32, 0, 16*16*4, 0, 0, 0, 0)
pixels = b'\xff\x80\x00\xff' * 256

script_dir = os.path.dirname(os.path.abspath(__file__))
ico_path = os.path.join(script_dir, 'icon.ico')

with open(ico_path, 'wb') as f:
    f.write(header + entry + dib + pixels)

print(f'Created {ico_path}')
