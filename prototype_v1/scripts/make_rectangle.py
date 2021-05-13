import math

# L = 1
# L = 0.5
# L = 0.25
L = 0.1

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

top_row = []
# first middle row
for _ in range(x_range):
    positions.append((x_pos,0.0,0.0))
    top_row.append(i)
    x_pos += L
    i += 1

z_step = L
z_pos = z_step

for z in range(z_range-1):
    x_pos = 0
    new_top = []
    for x in range(x_range):
        positions.append((x_pos, 0.0, z_pos))
        new_top.append(i)
        if x >= x_range - 1:
            i+=1
            continue
        positions.append((x_pos + L/2, 0.0, z_pos - L/2))

        faces.append([i, i + 1, top_row[x]])
        faces.append([i, i + 2 , i + 1])
        faces.append([i + 2, top_row[x+1], i+1])
        faces.append([top_row[x], i + 1, top_row[x+1]])
        i += 2

        x_pos += L
    top_row = new_top
    z_pos += z_step

print(f"Num verts = {len(positions)}")

with open("mesh.obj", "w") as f:
    for (x,y,z) in positions:
        f.write(f"v {x} {y} {z}\n")
    
    for (a,b,c) in faces:
        f.write(f"f {a+1} {b+1} {c+1}\n")