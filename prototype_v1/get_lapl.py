import argparse
import pyhks.hks
from pyhks.trimesh import load_off
import numpy as np 

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=str, required=True, help="Path to OFF file for triangle mesh on which to compute the HKS")

    opt = parser.parse_args()
    (VPos, VColors, ITris) = load_off(opt.input)
    L = pyhks.hks.get_cotan_laplacian(VPos, ITris)
    print("Laplacian")
    print(L)
    data = np.array([pos[0] **2 for pos in VPos])
    lapl = L * data
    for v, pos in zip(lapl, VPos):
        print(f"({pos[0]},{pos[1]},{pos[2]}) | {v}")

