# Functions

## Who is the Function?

Functions are organized instruction sets that may have a return value of a specific Type. Functions are decorated via the function signature. The function signature in Rust looks like so:

```rust

//  0 -
//  1    2  3         4         5   6   7   8
    pub fn funcName(parameter: Type) -> Type {

        // 9

        let dataValueMatchingSpecifiedType: Type = parameter;

        // 10    11
        return dataValueMatchingSpecifiedType; // 12 a.

        /*
         Comment out the code 12 a. and uncomment out the code 12 b.
         Notice what happens.
        */

        // dataValueMatchingSpecifiedType //12 b.
    }

    let arg: Type = "Hello World";

    // 14     //15
    funcName(arg); //16

    // 0. Function Signature
    // 1. Function Accessibility Keyword
    // 2. Function Declaration
    // 3. Function Name
    // 4. First Parameter
    // 5. First Parameter Specified Type
    // 6. Return Declarator
    // 7. Return Specified Type
    // 8. Body Block Bracket
    // 9. Function Body Block
    // 10. Return Keyword
    // 11. Data to be returned by the function
    // 12.
    // // a.) An Explicit Return Statement
    // // b.) An Implicit Return Statement
    // 13. Function Reference
    // 14. Supplied Argument
    // 15. Function Invocation
```

When you declare a function you are assigning a pointer in order to store data for it's return type, it's parameters, and any instructions within the body block of the function.

### Questions

1. Pair 13 of the 14 names with the proper Tokens concerning the given Function:

   ```Rust
    // []  []  []        []     []      []  []  []
       pub fn argReader(args: [String]) -> &str {
   //        []             []
           return "I read Arguments"; // []
       }
   ```

   - Token Symbols

     1. pub
     2. fn
     3. argReader
     4. args:
     5. [String]
     6. ->
     7. &str
     8. {
     9. return
     10. "I read Arguments";
     11. return "I read Arguments";
     12. { return "I read Arguments"; }
     13. pub fn Search(args[]) -> &str { return "I read Arguments"; }

   - Token Names

     1. Function Signature
     2. Function Accessibility Keyword
     3. Function Declaration
     4. Function Name
     5. Parameter
     6. Parameter Specified Type
     7. Return Declarator
     8. Return Specified Type
     9. Body Block Bracket
     10. Function Body Block
     11. Return Keyword
     12. Data to be returned by the function
     13. An Explicit Return Statement
     14. An Implicit Return Statement
