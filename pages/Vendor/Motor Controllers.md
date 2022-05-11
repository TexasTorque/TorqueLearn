# Motor Controllers

### What are PWM Motor Controllers?

Let’s say that you were to run a motor without any kind of motor controller, voltage setter, or anything, just simply plugged into a power supply. The motor would continuously speed up and up as it continued to get power and would not stay at a continuous, constant speed. This is obviously not ideal for many applications and robotics is no exception. Try and imagine an autonomous robot sequence where the robot has to drive forward at a certain speed, without a motor controller for the drive train the robot would speed up much too fast and likely crash into something hurting other people or itself.

Now you should be able to see that running a motor without any kind of controller would be very dangerous and that controllers are absolutely vital to the operation of the robot or any other motor-based appliance. But how exactly does a motor controller actually manage to keep a constant speed? The answer is pulse width modulation or PWM for short. The basic function of a PWM motor controller is to send small pulses of power to the motor to keep it going at a constant speed. Try and imagine turning on a switch for a motor and then quickly turning it off. The motor would have begun to speed up, and when you turned off the power, it would’ve begun to slow down. However, being able to perfectly control a motor through this means would be nearly impossible for a human, but because the motor controller is based on a computer, it is able to calculate the perfect length and strength of the pulses to keep the motor at a consistent speed. The pulses are not powerful or long enough to make the motor go too fast, nor are they too weak and short to make the motor underperform.

### Differences between motor controllers

What are the differences between different motor controllers? There are a wide variety of motor controllers that are used in different scenarios for different purposes. Here at Texas Torque, the motor controllers that are most commonly used are VictorSPX, Talon SRX, and RevRobotics SparkMax. The main practical difference between the three is the price and the electrical wiring between the controllers. Victors, SparkMaxs and Talons can be linked by CAN meaning that each motor controller is daisy-chained making the electrical debugging a bit more complex. On the programming side, CAN motors need PIDs, and you can take advantage of the “.addFollower” command for your motors. On the other hand, you can chose to not utilize CAN and instead directly link each controller to the RoboRIO, meaning that each motor has to be set individually when controlling/setting the speed. This method does not require any kind of PID to be used.

### Code

Here at Texas Torque, we utilize submodules to handle our vendordeps, so we can write out TorqueSparkMax to define a SparkMax, VictorSPX for VictorSPX's, and TalonSRX for TalonSRX's. To add a follower for a motor, simply rewrite the motor name on a new line with the .addFollower command afterwards with the motor that you wish to have be a follower in parenthesis. It is not neccesary to define the follower motor, it will automatically be done.

```java
TorqueSparkMax leftDB1 = new TorqueSparkMax(Ports.LEFT_DB_1);
leftDB1.addFollower(Ports.LEFT_DB_2);
```

### Problems and Solutions

* Problem: The motor is moving strangely (Rolling after input is false, stuttering, etc.)
    -Solution: Try to reconfigure the PID

## Credits

Initially written by [Omar Afzal](https://github.com/0mara) in October 2021

