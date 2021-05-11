import math

L = 1
# L = 0.5
# L = 0.25
height = 10
length = 10

faces = []
positions = []

z_range = int(height // L)
x_range = int(length // L)
print(x_range, z_range)

i = 0
x_pos = 0

top_row = []
bottom_row = []
# first middle row
for _ in range(x_range):
    positions.append((x_pos,0.0,0.0))
    top_row.append(i)
    bottom_row.append(i)
    x_pos += L
    i += 1

x_step = 0.5 * L
x_start = x_step
z_step = L * math.sqrt(3)/2
z_pos = z_step

for z in range(z_range):
    x_pos = x_start
    new_top = []
    new_bot = []
    num_x = x_range - (z+1)
    for x in range(num_x):
        positions.append((x_pos, 0.0, z_pos))
        new_top.append(i)

        positions.append((x_pos, 0.0, -1 * z_pos))
        new_bot.append(i + 1)

        if x > 0:
            faces.append((i-2, top_row[x], i))
            faces.append((i-1, bottom_row[x], i+1))

        faces.append((top_row[x], top_row[x+1], i))
        faces.append((bottom_row[x], bottom_row[x+1], i+1))
        i += 2
        x_pos += L

    top_row = new_top
    bottom_row = new_bot
    x_start += x_step
    z_pos += z_step

print(f"Num verts = {len(positions)}")
print(f"max x = {x_pos - 0.5 * L}")

with open("mesh.obj", "w") as f:
    for (x,y,z) in positions:
        f.write(f"v {x} {y} {z}\n")
    
    for (a,b,c) in faces:
        f.write(f"f {a+1} {b+1} {c+1}\n")