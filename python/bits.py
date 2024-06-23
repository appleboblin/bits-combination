def generate_bits(bit_length):
    # none if n = 0
    if bit_length == 0:
        return ['']

    # recursive base case, n-1
    new_combinations = generate_bits(bit_length - 1)
    combinations = []

    # add 0 and 1 for each combination
    for bits in new_combinations:
        combinations.append('0' + bits)
        combinations.append('1' + bits)

    return combinations

# input how many n
def main():
    # prompt
    bit_length = int(input("Number of bits: "))

    # combination generation
    combinations = generate_bits(bit_length)

    # print number of combinations
    print(f"Number of {bit_length}-bit combinations: {len(combinations)}")

    # print all combinations
    for bit in combinations:
        print(bit)

if __name__ == "__main__":
    main()