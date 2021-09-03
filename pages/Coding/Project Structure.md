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

The above code checks if we are pressing the left trigger on the controller. If we are, we are telling the robot to start the shooting sequence. Most of the input methods are structured like this, though not exactly. This block of code is called by the update() method. In general, this is how update() works:

``` 
update() (called 100 times per second) --> checks for controller input --> if input is being entered from the controller, the corresponding methods will control their respective motors.
``` 

## Subsystems

You saw above that when we press certain buttons on the controller, we want the robot to perform a certain tasks, but actually making it do the tasks is done in the Subsystems folder. Every subsystem has a corresponding file in this folder (Drivebase.java, Intake.java, etc.). Let's take a look at Shooter.java. You saw how pressing the left trigger sets the shootingSequence boolean to true. But, we still have to code what the robot is supposed to do. At the top of the file, we declared a variable called shooterLoader and set it to its port.

```java
shooterLoader = new PWMSparkMax(Ports.SHOOTER_LOADER);
```
Just like in the inputs file, there is an update() method in subsystems too. While in inputs, it checks for buttons being pressed, in subsystems, the update() method checks for booleans set to true in the inputs file. 




## Credits

Initially written by [Suhas Guddeti](https://github.com/Suhas44) in August 2021
