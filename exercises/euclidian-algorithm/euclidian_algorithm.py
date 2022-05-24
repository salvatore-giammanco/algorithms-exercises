def main():
    print("Great Common Divisor (GCD) Calculator!\n")

    first_number = int(input("Insert first number: "))
    second_number = int(input("Insert second number: "))
    module = max(first_number, second_number) % min(first_number, second_number)
    
    while module != 0:
        first_number = second_number
        second_number = module
        module = max(first_number, second_number) % min(first_number, second_number)
    
    print(f"GCD: {second_number}")

if __name__ == "__main__":
    main()