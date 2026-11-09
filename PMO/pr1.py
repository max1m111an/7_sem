import sympy as sp

x = sp.symbols('x', positive=True)

# 1
f1 = lambda x: (x**2 + sp.sqrt(x**2 - 2*x))**sp.Rational(1, 6)
f1_expected = lambda x: sp.Rational(1, 6) * (x**2 + sp.sqrt(x**2 - 2*x))**sp.Rational(-5, 6) * \
                        (2*x + (x - 1)/sp.sqrt(x**2 - 2*x))

# 2
u2 = lambda x: sp.sin(x - 1)**2 / (x**3 + 2*x + 5)
f2 = lambda x: 3 * sp.cos(u2(x))
u2_prime = lambda x: (sp.sin(2*(x - 1))*(x**3 + 2*x + 5) - sp.sin(x - 1)**2 * (3*x**2 + 2)) / (x**3 + 2*x + 5)**2
f2_expected = lambda x: -3 * sp.sin(u2(x)) * u2_prime(x)

# 3
f3 = lambda x: x * 2**(3*x + sp.log(x + 1))
f3_expected = lambda x: 2**(3*x + sp.log(x + 1)) * (1 + x*sp.log(2)*(3 + 1/(x + 1)))

# 4
f4 = lambda x: sp.log(15 * x**sp.sin(x + 5) + 13, 10)
f4_expected = lambda x: (15 * x**sp.sin(x + 5) * (sp.cos(x + 5)*sp.log(x) + sp.sin(x + 5)/x)) / \
                        ((15 * x**sp.sin(x + 5) + 13) * sp.log(10))

# 5
U = lambda x: 2 * sp.cos(14*x**4 - 5*x**3 + 12*x - 8)
V = lambda x: sp.sqrt(2 * sp.cos(x**2 - 2*x + 7))
f5 = lambda x: U(x) / V(x)
f5_expected = lambda x: (
    -2*sp.sin(14*x**4 - 5*x**3 + 12*x - 8)*(56*x**3 - 15*x**2 + 12)*sp.sqrt(2*sp.cos(x**2 - 2*x + 7))
    + 4*(x - 1)*sp.cos(14*x**4 - 5*x**3 + 12*x - 8)*sp.sin(x**2 - 2*x + 7)/sp.sqrt(2*sp.cos(x**2 - 2*x + 7))
) / (2*sp.cos(x**2 - 2*x + 7))

pairs = [(f1, f1_expected), (f2, f2_expected), (f3, f3_expected), (f4, f4_expected), (f5, f5_expected)]
test_points = [0.7, 1.3, 2.1, 3.4]

for i, (f, fe) in enumerate(pairs, 1):
    diff = sp.simplify(sp.diff(f(x), x) - fe(x))
    symbolic_ok = diff == 0
    numeric_ok = all(
        abs(complex(sp.diff(f(x), x).subs(x, p) - fe(x).subs(x, p))) < 1e-9
        for p in test_points
    )
    print(f"f{i}: symbolic={symbolic_ok}, numeric={numeric_ok}")
