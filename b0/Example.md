# Example for Syntax

```
/// function declare
@decorator(1, "abc")
test_function(arg1: U8, arg2: U8) U8 {
    // Declare variable and assign with type.
	a :u8 = 1;
	// Declare variable and assign.
	a := b;

    // Function Call
    call_func(1, a);

	// Assign value
	a = 1;

    // Assign function call
    a = call(a, 1, 2);

    // Assign string
    a = "";

    // Assign int
    a = 1;

    // Assign float
    a = 1.1

    // Assign hex
    a = 0xab;

    // If Statement
    a == 1 {
        // Some Statements Here
    } a == 2 {
        // Some Statements Here
    } b == 3 {
        // Some Statements Here
    } {
        // Else Statements
    }

    // For Statement
    (a:= 1; a < 8; a++) {
        // Statements
    }

    // Return
    <= 1
}
```