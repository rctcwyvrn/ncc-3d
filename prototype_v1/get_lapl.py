import argparse
import pyhks.hks
from pyhks.trimesh import load_off
import numpy as np 

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=str, required=True, help="Path to OFF file for triangle mesh on which to compute the laplacian")

    opt = parser.parse_args()
    (VPos, VColors, ITris) = load_off(opt.input)

    L = pyhks.hks.get_cotan_laplacian(VPos, ITris)
    # L = pyhks.hks.get_umbrella_laplacian(VPos, ITris)

    #print("Laplacian matrix")
    #print(L)

    data = np.array([pos[0]**2 for pos in VPos])
    lapl = L.dot(data)
    for v, pos, dat in zip(lapl, VPos, data):
        v = round(v, 5)
        dat = round(dat, 5)
        if pos[0] > 0.1 and pos[0] < 0.9 and pos[2] > 0.1 and pos[2] < 0.9:
            print(f"{pos[0]},{pos[1]},{pos[2]},{v},{dat}")
