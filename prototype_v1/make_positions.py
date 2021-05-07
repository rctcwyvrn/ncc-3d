import math

L = 1

faces = []
positions = []
positions.append((0.0,0.0,0.0))

x_pos = 0.5 * L
y_pos = L * math.sqrt(3)/2
i = 0

for _ in range(3):
    """
    i - 2        i + 1
            i           i + 3
    i - 1        i + 2 
    """

    faces.append((i, i + 1, i + 3))
    faces.append((i, i + 3, i + 2))

    positions.append((x_pos, y_pos, 0.0))
    positions.append((x_pos, -1 * y_pos, 0.0))
    positions.append((x_pos + 0.5 * L, 0.0, 0.0))

    if i > 0:
        # top middle face
        faces.append((i - 2, i + 1, i))

        # bottom middle face
        faces.append((i , i + 2, i - 1))

    i += 3
    x_pos += L

print(f"Num verts = {len(positions)}")
# positions = []
# positions.append((0.0,0.0,0.0))
# positions.append((x_pos, y_pos, 0.0))
# positions.append((x_pos + 0.5 * L, 0.0, 0.0))

# positions.append((0.0,0.0,0.0))
# positions.append((x_pos + 0.5 * L, 0.0, 0.0))
# positions.append((x_pos, -1 * y_pos, 0.0))

# fused_pos = []
# for p in positions:
#     for c in p:
#         fused_pos.append(c)

# fused_ind = []
# for p in faces:
#     for c in p:
#         fused_ind.append(c)

# print(f"let positions: Vec<f64> = vec!{fused_pos};")
# print(f"let indicies: Vec<u32> = vec!{fused_ind};")

with open("mesh.obj", "w") as f:
    for (x,y,z) in positions:
        f.write(f"v {x} {y} {z}\n")
    
    for (a,b,c) in faces:
        f.write(f"f {a+1} {b+1} {c+1}\n")