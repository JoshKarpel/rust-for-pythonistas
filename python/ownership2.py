def ownership2():
    foo = [1, 2, 3]
    bar = foo

    bar.append(4)

    print(foo)
    print(bar)


if __name__ == '__main__':
    ownership2()
