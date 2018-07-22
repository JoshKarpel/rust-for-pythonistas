def ownership():
    foo = 1
    bar = foo

    bar += 5

    print('foo ->', foo)
    print('bar ->', bar)


if __name__ == '__main__':
    ownership()
