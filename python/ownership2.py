def ownership2():
    foo = [1, 2, 3]
    bar = foo

    bar.append(4)

    print('foo ->', foo)
    print('bar ->', bar)


ownership2()
