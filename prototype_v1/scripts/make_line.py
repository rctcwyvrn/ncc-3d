import math

# L = 1
L = 0.5
# L = 0.25
# L = 0.1

# height = 0.5
length = 10

faces = []
positions = []

# z_range = int(height // L) + 1
z_range = 2
x_range = int(length // L) + 1
print(x_range, z_range)

i = 0
x_pos = 0

for _ in range(x_range):
    positions.append((x_pos,0.0,0.0))
    x_pos += L
    i += 1

print(f"Num verts = {len(positions)}")

with open("mesh.obj", "w") as f:
    for (x,y,z) in positions:
        f.write(f"v {x} {y} {z}\n")
    
    for (a,b,c) in faces:
        f.write(f"f {a+1} {b+1} {c+1}\n")