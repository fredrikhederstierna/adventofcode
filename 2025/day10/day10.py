
import sys

#
# >> B = [0 0 0 1; 0 1 0 1; 0 0 1 0; 0 0 1 1; 1 0 1 0; 1 1 0 0]
#
# B =
#   0   0   0   1
#   0   1   0   1
#   0   0   1   0
#   0   0   1   1
#   1   0   1   0
#   1   1   0   0
#
# >> P = [1 3 0 3 1 2]
# P =
#   1   3   0   3   1   2
#
# >> S = P * B
#
# S =
#   3   5   4   7
#
# >> sum(P)
#   ans = 10
#

from pulp import *

def solveProblem():
    prob = LpProblem("My_Problem_Name", LpMinimize) # Or LpMaximize
    prob += 0

    #x1 = LpVariable("X1", lowBound=0) # Variable X1 >= 0
    #x2 = LpVariable("X2", lowBound=0) # Variable X2 >= 0
    p1 = LpVariable("P1", lowBound=0, cat=LpInteger) # Variable P1 >= 0
    p2 = LpVariable("P2", lowBound=0, cat=LpInteger) # Variable P2 >= 0
    p3 = LpVariable("P3", lowBound=0, cat=LpInteger) # Variable P3 >= 0
    p4 = LpVariable("P4", lowBound=0, cat=LpInteger) # Variable P4 >= 0
    p5 = LpVariable("P5", lowBound=0, cat=LpInteger) # Variable P5 >= 0
    p6 = LpVariable("P6", lowBound=0, cat=LpInteger) # Variable P6 >= 0

    #prob += 2*x1 + 3*x2, "Total_Cost" # Minimize cost
    #prob += p1 + p2 + p3 + p4 + p5 + p6, "Total_Cost" # Minimize cost
    # Obj_f
    obj_f = pulp.LpAffineExpression()
    obj_f += p1
    obj_f += p2
    obj_f += p3
    obj_f += p4
    obj_f += p5
    obj_f += p6
    prob += obj_f, "Total_Cost"

    #prob += x1 + x2 <= 100, "Capacity" # Constraint 1
    #prob += 2*x1 - x2 >= 0, "Ratio"    # Constraint 2
    #
    #1: prob += p1 + p2 + p4 == 7, "Capacity1"  # Constraint 1
    #2: prob += p3 + p4 + p5 == 4, "Capacity2"  # Constraint 2
    #3: prob += p2 + p6      == 5, "Capacity3"  # Constraint 3
    #4: prob += p5 + p6      == 3, "Capacity4"  # Constraint 4

    #1
    expr = pulp.LpAffineExpression()
    expr += p1
    expr += p2
    expr += p4
    expr = (expr == 7)
    prob += expr, "Capacity 1"
    #2
    expr = pulp.LpAffineExpression()
    expr += p3
    expr += p4
    expr += p5
    expr = (expr == 4)
    prob += expr, "Capacity 2"
    #3
    expr = pulp.LpAffineExpression()
    expr += p2
    expr += p6
    expr = (expr == 5)
    prob += expr, "Capacity 3"
    #4
    expr = pulp.LpAffineExpression()
    expr += p5
    expr += p6
    expr = (expr == 3)
    prob += expr, "Capacity 4"

    prob.solve()

    print(LpStatus[prob.status]) # e.g., Optimal, Infeasible
    print(f"P1 = {value(p1)}")
    print(f"P2 = {value(p2)}")
    print(f"P3 = {value(p3)}")
    print(f"P4 = {value(p4)}")
    print(f"P5 = {value(p5)}")
    print(f"P6 = {value(p6)}")
    print(f"Sum = {value(p1)+value(p2)+value(p3)+value(p4)+value(p5)+value(p6)}")


def main() -> int:
    filename = 'test.in'
    #filename = 'input.txt'
    file = open(filename, mode = 'r', encoding = 'utf-8-sig')
    lines = file.readlines()
    file.close()

    for line in lines:
        line = line.strip()
        # extract parts
        pos_clam = line.find(']')
        pos_curly = line.find('{')
        s1 = line[pos_clam+1 : pos_curly]
        s2 = line[pos_curly+1 : len(line)-1]

        print(f'READLINE: {line} s1={s1} s2={s2}')

        butList = s1.split(')')
        print(f'butList.len = {len(butList)}')
        for b in butList:
            b = b.strip()
            b2 = b[1:]
            if (len(b2) > 0):
                print(f'but: {b2}')

        joltList = s2.split(',')
        print(f'joltList.len = {len(joltList)}')
        for j in joltList:
            print(f'jolt: {j}')

    solveProblem()

if __name__ == '__main__':
    sys.exit(main())  # next section explains the use of sys.exit
