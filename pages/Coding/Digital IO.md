# Using RoboRIO DIO

This is a tutorial on how to output the Digital I/O (DIO) pins on the RoboRIO, and accept input on an Arduino.

## On the RoboRIO
First, import the DigitalOutput class from wpilib as shown below.
```java
import edu.wpi.first.wpilibj.DigitalOutput;
```

Then, set up your instances
```java
DigitalOutput a;
DigitalOutput b;

private Constructor() {
    // DIO pin numbers, printed on RoboRIO
    a = DigitalOutput(0);
    a = DigitalOutput(1);
}
```

Then, in another method, set the state of your DIO pins with booleans.

```java

public method() {
    a.set(true);
    b.set(false);
}
```

## On the Arduino

To make sure we aren't constantly re-declaring variables, let's define some booleans globally.

```c++
bool thing;
bool that;
```


First in `setup`, set your pin modes
```c++
// pins here don't have to be the same
// as the ones on the RoboRIO
// as long as they are physically
// connected
pinMode(1, INPUT)
pinMode(2, INPUT)
```

Then in `loop` we can do what we need.

```c++
// make sure the pins are the same
// as the ones you set the mode
// on
thing = digitalRead(1)
that = digitalRead(2)
```

You can treat your read values as true/false, 1/0, or HIGH/LOW.