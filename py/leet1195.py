class FizzBuzz:
    def __init__(self, n: int):
        self.n = n
        self.i = 1
        self.queueFizz = queue.Queue()
        self.queueBuzz = queue.Queue()
        self.queueFizzBuzz = queue.Queue()
        self.queueNumber = queue.Queue()
        self.nextCond()

    def next(self):
        print(self.i)
        if self.i > self.n:
            for queue in [self.queueFizz, self.queueBuzz, self.queueFizzBuzz,
                          self.queueNumber]:
                queue.put('stop')
        elif self.i % 3 == 0 and self.i % 5 == 0:
            self.queueFizz.put('go')
        elif self.i % 3 == 0:
            self.queueBuzz.put('go')
        elif self.i % 5 == 0:
            self.queueFizzBuzz.put('go')
        else:
            self.queueNumber.put('go')

    # printFizz() outputs "fizz"
    def fizz(self, printFizz: 'Callable[[], None]') -> None:
        while True:
            got = self.queueFizz.get()
            if got == 'go':
                printFizz()
                self.i += 1
                self.next()
            else:
                return

    # printBuzz() outputs "buzz"
    def buzz(self, printBuzz: 'Callable[[], None]') -> None:
        while True:
            got = self.queueBuzz.get()
            if got == 'go':
                printBuzz()
                self.i += 1
                self.next()
            else:
                return

    # printFizzBuzz() outputs "fizzbuzz"
    def fizzbuzz(self, printFizzBuzz: 'Callable[[], None]') -> None:
        while True:
            got = self.queueFizzBuzz.get()
            if got == 'go':
                printFizzBuzz()
                self.i += 1
                self.next()
            else:
                return

    # printNumber(x) outputs "x", where x is an integer.
    def number(self, printNumber: 'Callable[[int], None]') -> None:
        while True:
            got = self.queueNumber.get()
            if got == 'go':
                printNumber(self.i)
                self.i += 1
                self.next()
            else:
                return
