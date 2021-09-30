# navX

The navX is an integrated accelerometer, gyroscope, and magnetometer that allows tracking the position of the robot.

![navX](/static/imgs/vendors/navx.png)

### Board

The board is located on the kMXP port on the RoboRIO, which is the large one in the center.

The navX has two on-board buttons, calibrate (CAL) and reset (RESET). Holding down reset for three seconds and waiting for the orange light to turn on and off will zero the gyro. Holding down CAL for 10 seconds and then pressing the reset button will recalibrate the sensors.

### Code

We use the AHRS class for interfacing with the navX. The gyro can be instantiated with:

```java
AHRS gyro = new AHRS(SPI.Port.kMXP);
```

Below is a list of common methods:

```java
double pitch = gyro.getPitch(); // Get the pitch
double yaw = gyro.getYaw(); // Get the yaw
double roll = gyro.getRoll(); // Get the roll

gyro.reset(); // Zero the gyro

gyro.getFusedHeading() // Get a more accurate yaw, [0,360]
```

See the section [Fused Heading](https://pdocs.kauailabs.com/navx-mxp/guidance/terminology/#:~:text=measure%20rotation%20similarly.-,%E2%80%9CFused%E2%80%9D%20Heading,-Given%20the%20gravity) for more in-depth details on the difference in headings.

### Problems & Solutions

* Problem: The gyro is giving wildly inaccurate values!
    - Solution: Have you tried recalibrating it?
