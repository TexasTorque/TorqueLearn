# Feed Forward

## Feed Forward versus Feedback

In [PID](/Tutorials/Control%20Loop/PID), we learned about a _feedback_ controller. Feedback in this case refers to how the control system responds to error. This works most of the time, however, this is _reactive_, not _proactive_. For a PID to work, your system must have error to begin with; which isn't ideal.

In contrast, a _feed forward_ controller uses model information to instantly set the approximate desired state. This reduces error time, and it can help minimize oscillations by reducing the PID's importance in the system.

## Why is in a Feed Forward Controller?

A normal feed forward controller takes in two to three values for the model: $k_s$ for static application, $k_v$ for velocity application, and (optionally for the most part) $k_a$ for acceleration application.

### What is a $k_s$ term needed?

It is important for those implementing control systems to have a healthy understanding of physics. As such, this will not go in depth for the reason.

In a nutshell, however, two connecting objects that contain a normal force exert a frictional force. There are two types of frictional forces, static and kinetic. Each type has its own coefficient of friction, μ. Static friction exists until a slipping point is reached, at which point kinetic friction takes over. This is due to intermolecular forces soft-binding objects at the subatomic level.

For many applications of PID, you may have witnessed the error term existing but nothing happening. This is due to static friction: despite putting in energy, the static friction is preventing movement. The $k_s$ term overcomes this by always keeping the model near or at the slipping point to provide fast changes in velocity.

## Calculating feed forward values

Feed forward values should **not** be calculated emperically. Proper characterization is needed. Additionally, dimensional analysis must be used to insure proper unit conversion, as output values are unit dependent.

## Credits

Initially written by [Jack Pittenger](https://github.com/realSaddy) in February 2022
