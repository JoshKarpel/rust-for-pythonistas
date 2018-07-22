def ownership():
    foo = 1
    bar = foo

    bar += 5

    print(foo)
    print(bar)


if __name__ == '__main__':
    ownership()
