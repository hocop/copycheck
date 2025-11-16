# Complex test file for copyedit-check
def calculate_values(data):
    # Rule R1: Identical RHS
    sum1 = data['total'] + data['bonus']
    sum2 = data['total'] + data['bonus']  # Duplicate calculation

    # Rule R5: Self-Assignment
    total = total  # Useless self-assignment

    # Rule R2: Repeated LHS
    result = data['value1'] * 2
    result = data['value2'] * 3  # Different operation but same LHS

    # Rule R7: Repeated Operand
    diff = var1 - var1  # Always zero

    # Rule R10: Multi-Increment
    matrix_sum = matrix[0][0] + matrix[1][0]  # Mismatched indices
    matrix_sum2 = matrix[0][1] + matrix[1][0]  # Another mismatch

    return sum1, sum2, result, diff, matrix_sum, matrix_sum2

class Processor:
    def __init__(self):
        # More complex patterns
        self.value = self.value  # Self-assignment in class init

    def process(self, input_data):
        # Identical RHS in method
        temp1 = input_data['x'] * 2
        temp2 = input_data['x'] * 2  # Same calculation

        return temp1 + temp2

# Test with nested structures
config = {
    'settings': {
        'option1': 'value1',
        'option2': 'value2'
    }
}

def process_config(cfg):
    # Repeated patterns in different contexts
    opt1 = cfg['settings']['option1']
    opt2 = cfg['settings']['option2']

    # Identical RHS across different levels
    result1 = opt1.upper()
    result2 = opt1.upper()  # Same transformation

    return result1, result2

