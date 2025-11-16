# Test file for copyedit-check
x1 = y + z
x2 = y + z  # R1: Identical RHS
x = y + z
x = a + b  # R2: Repeated LHS
diff = x - x  # R7: Repeated Operand
sum1 = a[0] + b[0]
sum2 = a[1] + b[0]  # R10: Multi-Increment
