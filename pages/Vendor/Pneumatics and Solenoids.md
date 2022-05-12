# Pneumatics and Solenoids

Pneumatics are pressurized air cylinders that are controlled by solenoids to either extend or retract. They are powered by compressors and can be used all over a robot to perform a variety of tasks. The difference between the two is that a double solenoid can be manually retracted from the extended position and also be set off, whereas a single solenoid will only be able to push out and when the solenoid is set to 0 it will retract.

## Programming

To program a pneumatic cylinder, you control the solenoid which controls the pneumatic device.

```java
import edu.wpi.first.wpilibj.Solenoid;

Solenoid solenoid = new Solenoid(PneumaticsModuleType.CTREPCM, Ports.SOLENOID);

if (controller.getSolenoidButton()) solenoid.set(true);
else solenoid.set(false);
```

```java
import edu.wpi.first.wpilibj.DoubleSolenoid;
import edu.wpi.first.wpilibj.DoubleSolenoid.Value;

DoubleSolenoid doubleSolenoid = new DoubleSolenoid(PneumaticsModuleType.CTREPCM, Ports.DOUBLE_SOLENOID_OUT, Ports.DOUBLE_SOLENOID_IN);

if (controller.getSolenoidButton()) doubleSolenoid.set(Value.kForward);
else doubleSolenoid.set(Value.kReverse);
```

![SolenoidWiringDiagram](/static/imgs/pneumatics-subsystem.png)

## Wiring

To wire your pneumatics, you will first need to have a basic electronics setup including a PCM (Pneumatics Control Module). You can reference [this article](https://docs.wpilib.org/en/stable/docs/zero-to-robot/step-1/how-to-wire-a-robot.html). The PCM will be used to provide an output for the compressor, input for the pressure switch, and outputs for up to 8 solenoid channels (12V or 24V selectable). The PCM is connected to the roboRIO over the CAN bus and powered via 12V from the PDP. If there are not enough ports on the PCM for a robot, and additional one can be used.

### Compressor

The compressor can be wired from the Compressor Out port on the PCM and should use no higher than 18 gauge wire.

### Pressure Switch

The polarity of the terminals for the pressure switch are not important when connecting them to the input ports on the PCM. A pressure switch is required to compete in a FRC competition as it is what prevents a compressor from over compressing and causing a rupture.

### Solenoids

Each solenoid channel should be wired directly to a numbered pair of terminals on the PCM. A single acting solenoid will use one numbered terminal pair. A double acting solenoid will use two pairs. If your solenoid does not come with color coded wiring, check the datasheet to make sure to wire with the proper polarity.

### Solenoid Voltage Jumper

The PCM is capable of powering either 12V or 24V solenoids, but all solenoids connected to a single PCM must be the same voltage. The PCM ships with the jumper in the 12V position. To use 24V solenoids move the jumper from the left two pins to the right two pins. You may need to use a tool such as a small screwdriver, small pair of pliers, or a pair of tweezers to remove the jumper.

## Problems and solutions

-   Problem: The solenoid light is turning on when the controller sends the signal, but the cylinder is not actuating.
    Solution: Did you change the Voltage Jumper to the correct voltage?
