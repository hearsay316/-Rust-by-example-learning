function fibonacci(n) {
    if (n == 0 || n == 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

console.time("a");
console.log(fibonacci(40));

console.timeEnd("a")