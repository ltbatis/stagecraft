# test.py

class Calculator:
    def add(self, a, b):
        return a + b

    def multiply(self, a, b):
        return a * b

    def complicated_function(self, x):
        if x > 0:
            if x % 2 == 0:
                return x / 2
            else:
                for i in range(x):
                    print(i)
        else:
            while x < 0:
                x += 1
        return x

def simple_function():
    return "Hello, world!"

def slightly_complex_function(x):
    if x > 10:
        return x * 2
    elif x > 5:
        return x + 5
    else:
        return x - 5

def deeply_nested_function(x):
    if x > 0:
        if x > 10:
            if x > 20:
                return "Huge"
            else:
                return "Big"
        else:
            if x > 5:
                return "Medium"
            else:
                return "Small"
    else:
        return "Negative"


def monstrous_function(x, y):
    if x > 0:
        if y > 0:
            if x > y:
                if x > 100:
                    return "Huge X"
                elif y > 100:
                    return "Huge Y"
                else:
                    for i in range(x):
                        if i % 2 == 0:
                            if i % 3 == 0:
                                continue
                            elif i % 5 == 0:
                                if i > 50:
                                    break
                    return "Loop completed"
            else:
                while y > 0:
                    y -= 1
                    if y % 2 == 0:
                        continue
                    elif y % 5 == 0:
                        if y < 30:
                            break
                return "Y decremented"
        else:
            if x < -10:
                return "Negative Large X"
            else:
                for j in range(abs(x)):
                    if j % 4 == 0:
                        if j % 6 == 0:
                            continue
                        else:
                            if j % 7 == 0:
                                break
                return "Negative X loop"
    else:
        if y < 0:
            if x + y == 0:
                return "Zero sum"
            elif x * y > 100:
                return "Large negative product"
            else:
                for k in range(abs(y)):
                    if k % 3 == 0:
                        if k % 5 == 0:
                            if k % 7 == 0:
                                return "Multiple of 3,5,7"
                            else:
                                return "Multiple of 3 and 5"
                        elif k % 11 == 0:
                            return "Multiple of 3 and 11"
                        else:
                            continue
                return "Y processed"
        else:
            return "Other case"
