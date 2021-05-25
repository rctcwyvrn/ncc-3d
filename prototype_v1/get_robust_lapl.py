import robust_laplacian
from scipy.sparse import linalg
import numpy as np 
from pyhks.trimesh import load_off
import argparse
import math

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=str, required=True, help="Path to OFF file for triangle mesh on which to compute the laplacian")

    opt = parser.parse_args()
    (VPos, VColors, ITris) = load_off(opt.input)

    L, M = robust_laplacian.mesh_laplacian(VPos, ITris)
    # L, M = robust_laplacian.point_cloud_laplacian(VPos) # only this one produces reasonable results... why?? the mesh is correct for SURE

    L = np.dot(linalg.inv(M), -1 * L)
    #print("Laplacian matrix")
    #print(L)

    # np.savetxt("robust_lapl_L.csv", L, delimiter=",")

    data = np.array([pos[0]**2 for pos in VPos])
    # data = np.array([ math.exp(pos[0]+pos[2]) for pos in VPos])

    lapl = L.dot(data)
    for v, pos, dat in zip(lapl, VPos, data):
        v = round(v, 5)
        dat = round(dat, 5)
        if pos[0] > 0.25 and pos[0] < 0.75 and pos[2] > 0.25 and pos[2] < 0.75:
            s = 2
            # s = 2 * math.exp(pos[0]+pos[2])

            print(f"{pos[0]},{pos[1]},{pos[2]},{v},{s},{abs(v-s)}")
