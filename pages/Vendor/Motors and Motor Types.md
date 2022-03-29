# Motors and Motor Types

### Brands and Models

As of the 2021 off-season, Texas Torque currently makes extensive use of RevRobotics NEO, NEO 550, VEX 775 pro, and is looking to incorporate the VEX Falcon 500 soon. In the past the team made use of the AndyMark CIM, but have since all but stopped using them in anything. All of these motors have features and quirks that make them suitable or unsuitable for specific roles.

### Brushless vs Brushed

These are the two main categories of DC motor. Both operate on the principals of electromagnetism and magnetism. The stator is the stationary part sourrounding the rotor, the rotating part.

In a brushed motor, the stator is made up of two oppositely charged magnets facing inwards towards the rotor. The rotor is made up of coils that function as electromagnets. Attached to the rotor is the commutator, a periodically interrupted cylinder of metal. 2 carbon brushes contact the commutator passing current through it and into the coils on the rotor, attracting them towards the stator, producing rotational motion. At some point during the rotation of the motor, the brushes lose contact with the commutator, stopping the flow of current through the rotor and sending the motor into free spin. Then the brushes re-initiate contact with the commutator, causing current to flow in the opposite direction, reversing the polarity of the rotor's electromagnetic coils, and yet again attracting it to the stator. This finely tuned mechanical process ensures that the motor will keep spinning smoothly in the same direction.

In a brushless motor, the rotor is still surrounded by a the stator, but the roles are reversed. The rotor is made up of a cylindrical magnet, attached to the load. An array of electromagnets point in from the stator towards the rotor (at least 3). A computer timing system switches the electromagnets on the stator on and off at the right time to continue attracting the rotor in the same direction.

For all intents and purposes, brushless motors are better then brushed motors. There are less durability concerns and less heat due to the lack of fragile brushes. They generally just perform better. For example, brushless motors have a higher torque-to-weight ratio.

### Performance Metrics

There are many ways to measure the performance of a motor.

The heat performance of a motor is important for motors that run all the time. Running a loaded motor for too long can cause quickly cause it to overheat, this is especially true for brushed motors. Be sure to monitor a motors temperature, otherwise you could have an poorly performing motor on your hands. The Falcon 500 comes with a dedicated pneumatic cooling port, which allows for the issue of heat to be almost entirely avoided.

A motor's efficiency can indicate how much of the 40A of current available throught the robot PDP that the robot can take advantage of.

Free spin speed is the maximum speed the motor will spin with no additional load. At this speed the motor is practically useless because torque (and therefore output power are 0).

Torque is the rotational equivalent of linear force (thx Wikepedia). An object that weighs 50 kg (ignoring friction) will take more than 50 kg of linear force to break equilibrium and the object moves. Torque can be conceptualized similarly. Force, but applied during a rotation. Torque is measured Newton-meters (Nm). 1 Nm is equal to the amount of force the rotational axis experiences when attached to 1 N (~100 g) at the end of a 1 meter long arm. The greater the torque, the less slowdown the motor will experience when under heavy load.

Output power is equal to torque times speed. Output power can be misleading. For example, a motor that spins extremely quickly but has almost no torque could still have a high output power, but be useless for most things. For FRC motors, generally the peak output power is good enough for comparing performance of motors in typical FRC applications.
