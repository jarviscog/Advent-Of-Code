
UNSET = 0
INCREASING = 1
DECREASING = 2

def is_safe(row: str) -> bool:
    elements = row.split(" ")
    asc_or_desc = UNSET
    for i in range(0, len(elements) - 1):

        diff = int(elements[i]) - int(elements[i+1])

        # within bounds
        if diff == 0 or abs(diff) > 3:
            return False

        # All asc, desc
        if diff > 0: # desc
            if asc_or_desc == INCREASING:
                return False
            asc_or_desc = DECREASING
        if diff < 0: # 
            if asc_or_desc == DECREASING:
                return False
            asc_or_desc = INCREASING

    return True


def main():
    
    sum_safe_rows = 0
    with open("./input.txt") as file:
        for line in file.readlines():
            if is_safe(line):
                sum_safe_rows += 1

    print(sum_safe_rows)

     
if __name__ == "__main__":
    main()
