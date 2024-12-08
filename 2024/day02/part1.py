
UNSET = 0
INCREASING = 1
DECREASING = 2

def is_safe(row: str):
    elements = row.split(" ")
    asc_or_desc = UNSET
    for i in range(0, len(elements)):

        if i == len(elements) - 1: # skip at last element 
            continue

        diff = int(elements[i]) - int(elements[i+1])

        # All asc, desc
        if diff == 0:
            return False
        if diff > 0: # desc
            if asc_or_desc == INCREASING:
                return False
            asc_or_desc = DECREASING
        if diff < 0: # 
            if asc_or_desc == DECREASING:
                return False
            asc_or_desc = INCREASING

        # within bounds
        diff = abs(diff)
        if diff < 1:
            return False
        if diff > 3:
            return False

    return True


def main():
    
    sum_safe_rows = 0
    with open("./input.txt") as file:
        for line in file.readlines():
            if is_safe(line):
                sum_safe_rows += 1
                print("safe")
            else:
                print("unsafe")


    print(sum_safe_rows)

     
if __name__ == "__main__":
    main()
