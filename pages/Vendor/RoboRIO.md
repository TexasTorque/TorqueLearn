# RoboRIO

## What is a RoboRIO?

A roboRIO is the only FRC legal reconfigurable robotics controller that includes built-in ports for inter-integrated circuits (I2C), serial peripheral interfaces (SPI), RS232, USB, Ethernet, pulse width modulation (PWM), and relays to quickly connect the common sensors and actuators used in robotics. A RoboRIO includes built in LEDs, buttons, an onboard accelerometer, and a custom electronics port and is the central hub for all other external sensors, motors, or lights to be wired to.

## Setting Up and Using WiFi: 

Out of the box, a RoboRIO is not fully set up as it does NOT include a built-in WiFi module, a part that is essential to controlling your robot remotely. Note: *It IS possible to run a RoboRIO with all functionality hardwiring to it over USB, you can deploy code and even enable the robot and have full motor and sensing functionality. However, you do need to have actual wireless connectivity to compete in an official FRC competition.* To interface with the RoboRIO over WiFi, you need to wire an external WiFi router and then configure it by connecting to the RoboRIOs ethernet port and using the WPILibs router imaging tool to set the robots network name(ER 1). Note: *It is necessary to disable all other network adapters on your PC besides the ethernet adapter when doing this*

## What Should Be Wired to a RoboRIO?

    The Power Distribution Panel (PDP), Pneumatics Control Module (PCM), Voltage Regulator Module (VRM), OpenMesh Radio, Robot signal light, motor controllers, 120A circuit breaker, WiFi radio, and battery connector are all parts that are needed to successfully and FRC legally wire a robot. However, not all of these are actually wired to the RoboRIO. The first step is to actually wire power to the RoboRIO and all that requires is to insert 10 and 20 amp fuses at the bottom of the PDP and then use 18 gauge wire to connect the “Vbat controller PWR” terminals on the PDP to the power input connector on the RoboRIO. For the WiFi radio, you need to first connect the radio to the 12V port on the VRM and then first connect a male to female ethernet adapter to the radio, and then a regular ethernet cable after that between the radio and the RoboRIO. For CAN wiring, simply connect one end of each wire to the CAN port on the RoboRIO and then the other ends to the correspondingly color coded ports on the PCM. For PWM devices, the most simple wiring method is to simply connect the PWM adapter to the motor controller (and if necessary PWM extension cables) and then connect the other end to one of the many PWM ports on the RoboRIO with the black wire facing out. For the signal light, first bridge the La and Lb terminals on the light, and then insert wire into the N(Negative) terminal and the Lb terminal and connect the two wires to the terminals corresponding positive and negative RSL port on the RoboRIO. (ER 2)

## Sources/Extra Resources

1. https://firstfrc.blob.core.windows.net/frc2020/Radio/FRC_Radio_Configuration_20_0_0.zip

2. https://docs.wpilib.org/en/stable/docs/zero-to-robot/step-1/how-to-wire-a-robot.html
