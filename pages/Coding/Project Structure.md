# Project Structure

To control and code the robot, you should know where the critical components are. There are 3 main folders: Constants, Inputs and Subsystems.

### Constants

The Constants folder has 2 files under it: Constants.java and Ports.java. Every motor on the robot is connected to a port. Ports.java is where we specify which motor is connected to which port. Constants.java has standard constants we use throughout the project, most notably the speeds of the motors

### Inputs

The Inputs folder has the Inputs.java file. First, at the top of the input file, are where the default speeds and booleans are set, usually 0 and false repectively. We make a variable called controller, which represents our Xbox One controller. With controller, we can actually see what buttons are being pressed, and we can specify what the robot should do if a specific button is pressed. 

```java
boolean shootingSequence = false;

if (controller.getLeftTrigger()) {
    shootingSequence = true;
}
```

The above code checks if we are pressing the left trigger on the controller. If we are, we are telling the robot to start the shooting sequence. Most of the input methods are structured like this, though not exactly.

## Credits

Initially written by [Suhas Guddeti](https://github.com/Suhas44) in August 2021
