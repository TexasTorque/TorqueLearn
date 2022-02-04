# PID

PID _(Proportional-Integral-Derivative)_ is a continuous feedback loop that attempts to reach a certain real-life encoder value.

## Vocabulary

- **Error (e)** - The error of a system is the amount that the system is off of the requested value. For example, if we wish to be at 500m but are at 100m, then we have an error of 400m.
- **Encoder value** - An encoder value is a measurement recorded using a real-life recording device. For example, an encoder on a wheel can record how many times it has revolved.
- **Zero** - A zero error / zeroed value means that the system is exactly where we requested. While possible in theory, you will almost never hit this in the real world.

## Proportional

The Proportional part of the PID controller influences the output of the controller by a proportion of the error.

$$p = k_p*e$$

For example, let's say we have an error value of 10. If we have a proportional value of 5 (often stylized `Kp`), then it will output 50 (10\*5). Note that this often causes a system to _overshoot_ by far exceeding the requested point, and then conversely _undershoot_ on the way back down.

## Integral

The Integral part of the PID controller increases the force applied as time passes without reaching zero error. It behaves like an integral in calculus.

$$i = \int_{0}^{t} e*k_idt$$

## Derivative

The Derivative part of the PID controller is linked to the rate of change of the error. It behaves like the derivative of a continuous function in calculus.

$$d = \frac{de}{dt}*k_d$$

## Output

The output of a PID is the summation of the p, i, and d terms.

$$output = p + i + d$$

## Credits

Initially written by [Jack Pittenger](https://github.com/realSaddy) in July 2021
