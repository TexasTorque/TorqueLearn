# Programming style and best practices

## Important Advice

- Simple > Complicated
- Explicit > Implicit
- Ability to read code > Ability to write code
- Comments don't explain your code to humans, code explains your comments to the computer
- The best procedure is no procedure
- Don't overuse a programming paradigm (OOP, functional)
- It's not what you know, it's what you do with what you know
- Leet Code doesn't mean much of anything
- The most valuable skill is the ability to figure things out
- Before you ask what something does, think through it
- That said, if you are really stuck, *please* ask questions

## Best Practices

### Use getters and setters

Avoid unnecessary public variables, getters, and setters, but use getters and setters to control access to private members.

### Curly braces go on the same line as the statement they are tied to

Except for really long function declarations

```java
public class Example {
    public static void main(String[] args) {
        if (args.length > 5) {
            System.out.printf("Hello World!\n");
        }
    }
}
```

### Lines should not extend over 80 characters

Examples of how to deal with long lines. This doesn't matter too much, you can do it many ways.

```java
public static ArrayList<String> reallyLongFunction(double thisArgument, 
        const String thatArgument, ThisClass anArgument) {

}

// This is one of the only time to put 
// the curly brace on the next line
private ArrayList<double>
 anotherFunction(double thisArgument, 
                 const String thatArgument, 
                 ThisClass anArgument) 
{

}
```

### Constants are named in screaming case

```java
String THIS_IS_SCREAMING_CASE = "Helicopter";
```

### Classes start with a capital letter, variables start with lower case

```java
ThisIsAClass thisIsAVariable = new ThisIsAClass();
```

### Use JavaDoc when you can

```java
/**
 * This says what the functions does
 * 
 * @param int This describes the parameter
 * @return This describes the return
 * 
 * @author Just don't use this w/o asking
 */
int thisIsAFunction(int a) {
    return -a;
}
```

## Credits

Initially written by [Justus Languell](https://github.com/Juicestus) in August 2021
