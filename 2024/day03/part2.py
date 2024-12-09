
def main():

    with open("./input.txt") as file:
        contents = file.read()

    EXPECTING_START = 1
    EXPECTING_INT_1 = 2
    EXPECTING_INT_2 = 3
    EXPECTING_END = 4

    current_state = EXPECTING_START
    current_token = ""
    num_one = 0
    num_two = 0
    sum = 0
    do = True
    for c in contents:
        current_token += c
        print("")
        print(f"Current token: [{current_token}]       State: [{current_state}]")
        # print("C            : [" + c + "]")

        if current_state == EXPECTING_START:
            if len(current_token) < len("mul("):
                continue
            if current_token[-4:-1]+current_token[-1] == "mul(": # This is really dumb
                print("  found [mul(]")
                current_state = EXPECTING_INT_1
                current_token = ""
                continue
            if len(current_token) > 3 and current_token[-4:-1]+current_token[-1] == "do()":
                print("do")
                current_token = ""
                do = True
                continue
            if len(current_token) > 6 and current_token[-7:-1]+current_token[-1] == "don't()":
                print("don't")
                current_token = ""
                do = False
                continue

        elif current_state == EXPECTING_INT_1:
            print("TOKEN: " + current_token)
            if len(current_token) == 0:
                continue
            if c == ',':
                print(f"  num_one: [{current_token.rstrip(',')}]")
                num_one = int(current_token.rstrip(','))
                current_state = EXPECTING_INT_2
                current_token = ""
                continue
            if not c.isdigit():
                current_token = ""
                current_state = EXPECTING_START
                continue

        elif current_state == EXPECTING_INT_2:
            print("TOKEN: " + current_token)
            if len(current_token) == 0:
                continue
            if c == ')':
                print(f"  num_two: [{current_token.rstrip(')')}]")
                num_two = int(current_token.rstrip(')'))
                current_token = ""
                current_state = EXPECTING_END
                continue
            if not c.isdigit():
                current_token = ""
                current_state = EXPECTING_START
                continue
        elif current_state == EXPECTING_END:
            if do == True:
                sum += num_one * num_two
            num_one = 0
            num_two = 0
            current_state = EXPECTING_START



    print(f"Sum: {sum}")


if __name__ == "__main__":
    main()
