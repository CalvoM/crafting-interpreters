# Language design notes

This are the design notes for the language will be building.

## High-level language

- **Typing** - It is dynamic typing.
- **Memory management** - We will use Automatic Reference Count.

## Data Types

- Booleans - (**true** and **false**).
- Numbers - (**integers** and **floats**)
- Strings - double quotes enclosed
- nil

## Expressions (Produces a value)

- Arithmetic - **Addition**(+), **Subtraction**(-), **Multiplication**(*), **Division**(/) (Applicable to numbers, apart from **+** which can be used with strings).
- Comparison and Equality - Comparison - **< <= > >=** and Equality **==, !=**. (Used and evaluate on same types mostly).
- Logical operators - using **not** and **!** prefix returns **false**. Using **and** and **or** operands.
- Precedence and grouping - Uses same precedence and associativity as C programming.

## Statements (Produces an effect)

- Terminated with semi-colons;
- An expression terminated with a **;** is technically called an **expressin-statement**.
- We have **blocks** for more than one statements. Blocking affects scoping.

## Variables

- Declare variables using **var** statements.
- Omiting the initializer, defaults the variable to **nil**.

## Control Flow

- We have the **if**, **while** and **for** statements

## Functions

- Are first class. They are real values and you can get references.
- Expression as used in C.
- Uses **fun** statement and has **return** statement. A function without a return statement, returns **nil**.
```js
fun function_name(<par1>, <par2>) {
    return <return_value>
}
```

## Closures

- We have closures.

## Classes

- They are first class.
```js
class Breakfast {
    cook() {
        print "Eggs a-frying!";
    }

    serve(who) {
        print "Enjoy your breakfast, " + who + ".";
    }
}

// instantiation
var breakfast = Breakfast();
print breakfast;

// class fields/properties
breakfast.meat = "sausages"; // creates if it does not exist
breakfast.bread = "sourdough";
```

- Constructor method is called **init** and uses **this** for internal instance variable.
- We have single inheritance and use **<** e.g.
```js
class Child < Parent {}
```
- We have **super** keyword to refer to the parent/base class.
```js
class Brunch < Breakfast {
    init(meat, bread, drink) {
        super.init(meat, bread);
        this.drink = drink;
    }
}
```

## Standard Library

- Not implemented.