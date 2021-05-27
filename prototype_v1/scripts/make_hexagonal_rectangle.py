import math

# L = 1
# L = 0.5
L = 0.03
# L = 0.01
# L = 0.005

height = 1
length = 1

faces = []
positions = []


x_step = 0.5 * L
z_step = L * math.sqrt(3)/2
z_pos = z_step


z_range = int(height // z_step) + 1
x_range = int(length // L) + 1
print(x_range, z_range)

i = 0
x_pos = 0

top_row = []
bottom_row = []
# first middle row
for _ in range(x_range):
    positions.append((x_pos,0.0,0.0))
    top_row.append(i)
    x_pos += L
    i += 1

odd_row = True
for z in range(z_range):
    if odd_row:
        x_pos = x_step
        num_x = x_range - 1
    else:
        x_pos = 0
        num_x = x_range

    new_top = []
    for x_i in range(num_x):
        positions.append((x_pos, 0.0, z_pos))
        new_top.append(i)

        if x_i > 0:
            if odd_row:
                faces.append((i-1, top_row[x_i], i))
            else:
                faces.append((i-1, top_row[x_i-1], i))

        if (x_i > 0 and x_i < num_x - 1) or odd_row:
            if odd_row:
                faces.append((top_row[x_i], top_row[x_i+1], i))
            else:
                faces.append((top_row[x_i-1], top_row[x_i], i))
        

        i += 1
        x_pos += L

    odd_row = not odd_row
    top_row = new_top
    z_pos += z_step

print(f"Num verts = {len(positions)}")
print(f"Num faces = {len(faces)}")


with open("mesh.obj", "w") as f:
    for (x,y,z) in positions:
        f.write(f"v {x} {y} {z}\n")
    
    for (a,b,c) in faces:
        f.write(f"f {a+1} {b+1} {c+1}\n")


with open("mesh.off", "w") as f:
    f.write("OFF\n")
    f.write(f"{len(positions)} {len(faces)} 0\n")
    for (x,y,z) in positions:
        f.write(f" {x} {y} {z}\n")
    
    for (a,b,c) in faces:
        f.write(f"3 {a} {b} {c}\n")