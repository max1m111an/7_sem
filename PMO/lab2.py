import numpy as np

mat = np.array([
    [54, 87, 89, 45],  # A1
    [74, 53, 79, 83],  # A3
    [88, 70, 72, 45],  # A4
])

n, m = mat.shape

print("Start matrix A")
print(mat)

col_max = mat.max(axis=0)
R = col_max - mat

print("\nRisks matrix R")
print(R)

W_Wald = mat.min(axis=1).max()
print("\nWald's Maximin criterion")
print(f"W = {W_Wald}")

W_Savage = R.max(axis=1).min()
print("\nSavage's Minimax Risk Criterion")
print(f"W = {W_Savage}")

gamma = [0.25, 0.5, 0.75]
print("\nHurwitz's criterion of pessimism-optimism")
for g in gamma:
    _set = (g * mat.min(axis=1) + (1 - g) * mat.max(axis=1))
    W_Hurwitz = _set.max()
    print(f"W({g}) = {W_Hurwitz}\t{_set}")

W_Laplace = (mat.sum(axis=1) / m).max()
print("\nLaplace's criterion")
print(f"W = {W_Laplace}")

print("\nIDEAL EXP")
Q = np.array([0.5, 0.2, 0.1, 0.2])
C = 21
print("Possibilities vector Q")
print(Q)

a_no_exp = float('-inf')
for i in mat:
    tmp = (i * Q).sum()
    print(tmp)
    if a_no_exp < tmp.sum():
        a_no_exp = tmp.sum()

M_avg = np.sum(Q * col_max)
a_id_exp = M_avg - C

print(f"\nNo exp = {a_no_exp} <> Ideal_exp = {a_id_exp}")

r_min_avg = float('inf')

for i in R:
    tmp = (i * Q).sum()
    print(i, '\t', tmp)
    if r_min_avg > tmp:
        r_min_avg = tmp

print(f"r_avg = {r_min_avg:.5} <> C = {C}")

print(a_id_exp > a_no_exp)