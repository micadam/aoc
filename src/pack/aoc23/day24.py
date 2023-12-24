from sympy import symbols, Eq, solve

with open("./input/aoc23/day24.txt") as f:
    lines = f.readlines()
  
equations = []

X, Y, Z, VX, VY, VZ = symbols("X Y Z VX VY VZ", real=True)

for i, line in enumerate(lines):
    pos, v = line.strip().split(" @ ")
    x_i, y_i, z_i = map(int, pos.split(", "))
    vx_i, vy_i, vz_i = map(int, v.split(", "))
    T_I = symbols(f"T_{i}", negative=False, real=True, zero=False)
    equations.append(Eq((VX - vx_i) * T_I + X, x_i))
    equations.append(Eq((VY - vy_i) * T_I + Y, y_i))
    equations.append(Eq((VZ - vz_i) * T_I + Z, z_i))
print(equations)
print("Solving...")
sol = solve(equations)
print(sol)
print(sol[0][X] + sol[0][Y] + sol[0][Z])
