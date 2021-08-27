# Programming style and best practices

## Important Advice

- Simple > Complicated
- Explicit > Implicit
- Comments don't explain your code to humans, code explains your comments to the computer
- The best procedure is no procedure
- Only use Object Oritentation when necessary
- It's not what you know, it's what you do with what you know
- Leet Code doesn't mean much if anything
- The most valuable skill is the ability to figure things out
- Before you ask what something does, think through it
- That said, if you are really stuck, *please* ask questions

## Best Practices

### Avoid using getters and setters

They lead to over encapsulation and are generally not necessary.

### Curly braces go on the same line as the statement they are tied to

Except for really long function declerations

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
