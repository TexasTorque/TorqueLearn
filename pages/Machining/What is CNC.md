# What is CNC machining?

CNC machining is a subtractive process (meaning material is removed) in which a spindle, that moves relative to the part that is being cut, spins an endmill that cuts away material from stock (the raw material) to achieve the desired geometry.

## Why is CNC machining important?

CNC machining allows us to quickly make very accurate custom parts. Many custom parts are needed for the robot that we would have no way of purchasing. Without our CNC machines, we would be required to manually fabricate the parts, which would be very inaccurate and time inefficient, or purchase parts from a machine shop, which is often costly and slow.

## How does the machine know what to do?

It doesn't. The CNC machine only follows the instructions we give it through a file type called Gcode. This Gcode is generated through software called CAM. As the CNC lacks knowledge of what is doing, we must be careful when generating CAM and setting origins for the machine (telling it where to cut) to ensure the machines performs the desired actions and doesn't crash into the part or work piece.

## Credits

Initially written by [Max Volanski](https://github.com/JJ48) in February 2022
