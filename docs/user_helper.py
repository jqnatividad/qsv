# example python helper file that can be used with the py command
# qsv map --helper user_helper.py fib qsv_uh.fibonacci(num_col) data.csv
# qsv map --helper user_helper.py fahrenheit qsv_uh.celsius_to_fahrenheit(celsius) data.csv
def fibonacci(input):
    try:
      float(input)
    except ValueError:
      return "incorrect input - not a number"
    sinput = str(input)
    if not float(sinput).is_integer():
        return "incorrect input - not a whole number"

    n = int(sinput)
    if n < 0:
        return "incorrect input - negative number"
    elif n == 0:
        return 0
    elif n == 1 or n == 2:
        return 1
    else:
        return fibonacci(n-1) + fibonacci(n-2)


def celsius_to_fahrenheit(celsius):
    try:
      float(celsius)
    except ValueError:
      return "incorrect input - not a float"
    fahrenheit = (float(celsius) * 9/5) + 32
    return f'{fahrenheit:.1f}'
