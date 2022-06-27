//
// Problem:
// In Go constants are just what the name suggests, constant- unchangeable,
// immutable.
// Go supports constant string, numeric, boolean values.
//
// You can't modify or reassign to a constant in Go.

package main

import "fmt"
import "math"

func main() {
	// In Go variable names must start with a letter.
	const twentynumber = 20
	fmt.Println(math.Sin(twentynumber)) 
	
	const abc = "abc"
	fmt.Println(abc)

	const sum = abc + "tyk"
	fmt.Println(sum)

	// Something's wrong here- an attempt to 
	// assign to a constant will cause a compiler error.
	// Instead of assigning to an existing constant, declare a new one!
	const newsum = sum + "cheeky addition"
	fmt.Println(newsum)
}
