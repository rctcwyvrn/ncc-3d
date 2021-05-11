import math 

# https://www.csee.umbc.edu/~squire/reference/polyhedra.shtml#icosahedron

pi = math.pi
phiaa = 26.56505
verts = [(-1,-1,-1) for _ in range(12)]
r = 5
phia = pi * phiaa / 180
theb = pi * 36 / 180
the72 = pi * 72 / 180

verts[0] = [0, 0, r]
verts[11] = [0,0, -r]

the = 0
for i in range(1,6):
    verts[i] = [r*math.cos(the)*math.cos(phia), r*math.sin(the)*math.cos(phia), r*math.sin(phia)]
    the += the72

the = theb
for i in range(6, 11):
    verts[i] = [r*math.cos(the)*math.cos(-phia), r*math.sin(the)*math.cos(-phia), r*math.sin(-phia)]
    the += the72

faces = []
faces.append([0,1,2])
faces.append([0,2,3])
faces.append([0,3,4])
faces.append([0,4,5])
faces.append([0,5,1])
faces.append([11,6,7])
faces.append([11,7,8])
faces.append([11,8,9])
faces.append([11,9,10])
faces.append([11,10,6])
faces.append([1,2,6])
faces.append([2,3,7])
faces.append([3,4,8])
faces.append([4,5,9])
faces.append([5,1,10])
faces.append([6,7,2])
faces.append([7,8,3])
faces.append([8,9,4])
faces.append([9,10,5])
faces.append([10,6,1])

with open("mesh.obj", "w") as f:
    for (x,y,z) in verts:
        # add r to x to shift it over from [-r, r] to [0, 2r]
        f.write(f"v {x+r} {y} {z}\n")
    
    for (a,b,c) in faces:
        f.write(f"f {a+1} {b+1} {c+1}\n")