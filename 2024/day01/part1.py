




def main():
    
    with open("./input.txt") as file:
        contents = file.readlines()

    arr1 = []
    arr2 = []
    for line in contents:
        arr1.append(int(line.split("   ")[0]))
        arr2.append(int(line.split("   ")[1]))

    print("First of each")
    print(arr1[0])
    print(arr2[0])

    arr1.sort()
    arr2.sort()

    diff_sum = 0
    for i in range(0,len(arr1)):
        diff_sum += abs(arr1[i] - arr2[i])

    print(diff_sum)



if __name__ == "__main__":
    main()
