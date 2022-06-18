# Motor Controllers

### What are PWM Motor Controllers?

Let’s say that you were to run a motor without any kind of motor controller, voltage setter, or anything, just simply plugged into a power supply. The motor would continuously speed up and up as it continued to get power and would not stay at a continuous, constant speed. Try and imagine an autonomous robot sequence where the robot has to drive forward at a certain speed, without a motor controller for the drive train the robot would speed up much too fast and likely crash into something hurting other people or itself.

How exactly does a motor controller actually manage to keep a constant speed? The answer is pulse width modulation or PWM for short. The basic function of a PWM motor controller is to send small pulses of power to the motor to keep it going at a constant speed. It is similair to turning on a switch for a motor and then quickly turning it off. Being able to perfectly control a motor through this means would be nearly impossible for a human, but because the motor controller is based on a computer, it is able to calculate the perfect length and strength of the pulses to keep the motor at a consistent speed. The pulses are not powerful or long enough to make the motor go too fast, nor are they too weak and short to make the motor underperform.

### Differences between motor controllers

What are the differences between different motor controllers? There are a wide variety of motor controllers that are used in different scenarios for different purposes. The motor controllers that are most commonly used are CTRE VictorSPX, VictorSP, and Talon SRX, and RevRobotics SparkMax. The main practical difference between the three is the price and the electrical wiring between the controllers. VictorSPXs are smaller and lighter than SPs and support CAN. SparkMax's, and Talons can also be linked by CAN and support encoders.

### Code

Here at Texas Torque, we utilize wrapper classes for motor controllers to handle our vendordeps, so we can write out TorqueSparkMax to define a SparkMax, VictorSPX for VictorSPX's, and TalonSRX for TalonSRX's. But for other teams you can define your controller object class with either a PWMSparkMax, CANSparkMax, or TalonSRX, VictorSP, or VictorSPX. Torques wrapper classes allows us to add a follower to a motor, so simply rewrite the motor name on a new line with the .addFollower command afterwards with the motor that you wish to have be a follower in parenthesis. It is not neccesary to define the follower motor, it will automatically be done.

```java
TorqueSparkMax leftDB1 = new TorqueSparkMax(Ports.LEFT_DB_1);
leftDB1.addFollower(Ports.LEFT_DB_2);
```

### Problems and Solutions

-   Problem: The motor is moving strangely (Rolling after input is false, stuttering, etc.)
    -Solution: Try to reconfigure the PID

## Credits

Initially written by [Omar Afzal](https://github.com/0mara) in October 2021
