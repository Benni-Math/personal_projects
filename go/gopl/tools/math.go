// small tool for calculating greatest common divisor via Euclid's Algorithm
package tools

// Euclid's Algorithm - find the greatest common divisor of two numbers
func GCD(x, y int) int {
    for y != 0 {
        x, y = y, x%y
    }
    return x
}

// Get the nth Fibonacci number
func Fib(n int) int {
    x, y := 0, 1
    for i := 0; i < n; i++ {
        x, y = y, x+y
    }
    return x
}

