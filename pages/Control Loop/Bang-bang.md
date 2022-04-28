# Bang-bang

Too lazy to make a PID? Bang-bang is perfect for you.

Basically, the idea is to keep doing something until you're there, stop when you're there, and then repeat.

Imagine you have a variable `pos` that is the current position of something

```java
public void bang_bang(pos: int) {
    if pos != 100 {
        // Move to position
    } else {
        // stop
    }
}
```