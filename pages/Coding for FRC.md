# Coding for FRC

**NOTE: This is *very* unfinished, sorry...**

Before we program a robot, we are going to need a tour of the FRC Programming Enviorment. FRC uses a library called [WPILib](https://docs.wpilib.org/en/stable/) to manage the enviorment.

## How to use WPILib

The version of Visual Studio Code that you installed earlier has WPILib built in. With the WPILib button in the top-right of the window, you have access to commands for writing, building, and deploying robot code.

## Making a New Project

Before you make a new project, make sure you or someone on your team has experience in the [Java](https://www.w3schools.com/java/) programming language. You can also program FRC Robots with C++, but Java is generally easier for development, and has a negligible diffrence in performance. 
To make a project, you click the WPILib button, and select ``WPILib: Create a new project``. This will open up the project wizard, allowing you to select a language. This will make the new project with all the correctly formatted build files, allowing you to build and deploy code using the WPILib button and commands. You can also use the wizard to generate example projects, which can be great for observing the way the projects are built.

**NOTE: Assume Java from now on!**

## Project Layout

In a project, there is are two important auto-generated files: Main.java, and Robot.java.

* Main.java calls Robot.java, just leave it alone.
* Robot.java has all the methods called by the RoboRIO firmware.

These functions are where your code goes. At Texas Torque, we have a way of organizing systems into diffrent classes and managing singleton instances, but hypothetically, you could just write code in the methods.

An easy way of thinking about these methods is with [Arudino](https://www.arduino.cc/). In an Arduino sketch, there are two default methods. They look like this:

```cpp
// Arduino sketches are a version of C++

// Called on initialization
void setup() {
    // Serial.print(); is the method to print to Arduino console
    Serial.print("This is called once at the beginning");
}

// Called on every update in the loop
void update() {
    Serial.print("This is called every time the Arudino loops");
}
```

In the FRC code, there are different types of these functions for the different states the robot can be in.