import sympy as sp

# Переменные
x1, x2, x3 = sp.symbols('x1 x2 x3', real=True)

# Функция
f = x1**4 - 4*x1**2 + x2**4 - 2*x2**2 + x3**2

# Градиент
grad = [sp.diff(f, v) for v in (x1, x2, x3)]
print("Gradient:")
for g in grad:
    print(" ", sp.simplify(g))

# Стационарные точки
solutions = sp.solve(grad, [x1, x2, x3], dict=True)
print(f"\nStationary points: {len(solutions)}")
for s in solutions:
    val = f.subs(s)
    print(f"  {s}  ->  f = {val}")

# Гессиан
H = sp.Matrix([[sp.diff(f, a, b) for b in (x1, x2, x3)] for a in (x1, x2, x3)])
print("\nHessian:")
sp.pprint(H)

# Классификация каждой точки
print("\nClassification:")
for s in solutions:
    H_num = H.subs(s)
    eigenvals = H_num.eigenvals()
    signs = [sp.sign(e) for e in eigenvals]
    if all(e > 0 for e in eigenvals):
        kind = "Loc min"
    elif all(e < 0 for e in eigenvals):
        kind = "Loc max"
    elif any(e > 0 for e in eigenvals) and any(e < 0 for e in eigenvals):
        kind = "Saddle"
    else:
        kind = "undef"
    print(f"  {s}: own numbers {list(eigenvals.keys())} -> {kind}, f = {f.subs(s)}")