# Gears

## Intro

Gear are a simple yet effective way to trasfer rotaional power. Gears are used in all sorts of mechanisms from drivetrains to jointed arms and elevators.

## Concepts

- [Positioning](#positioning)
- [Reductions](#reductions)
- [Inertia](#inertia)

### Positioning

**Relevance:** Gears that are positioned too close togehter can bind up and cause teeth to wear away faster.

Gears have an intrinsic property known as diamerial pitch _dp_. The diametrial pitch is defined as the the number of teeth on a gear _T_ divided by the pitch diamter _d_. The pitch diameter is the diameter of a circle that intersects the teeth on the gear in the middle of the tooth height (technically it is when the tooth width is the same as the spacing between the teeth). Pitch diameter is easier explained with an image:

![Thread Diagram](/static/imgs/CAD/ThreadDiagram.gif)

Finding the diameter of a gear from dp:

$$dp=\text{teeth}/\text{diamter}$$
$$dp=T/d$$
$$d=T/dp$$

The recommended distance _rd_ between gears is the center distance _cd_ plus 0.008 inches.

$$ cd=\frac{T\_{1}+T\_{2}}{2dp} $$

$$ rd=\frac{T\*{1}+T\*{2}}{2dp}+0.008$$

### Reductions

**Relevance:** Gears can alter the speed, direction, and torque of an input. Understanding reductions provides insight on how well a system can perfom a task.

#### Rotational Velocity

Gear are typically arranged in two ways: side by side, or stacked.

![Gear Arrangement Diagram](/static/imgs/CAD/GearArrangementDiagram.gif)

When gears are stacked, they have the same rotaional velocity $\omega$. When gears are side by side, they have the same tangental speed $|v|$.

A reduction occurs when a small gear "drives" a bigger gear aka. a small gear is next to a bigger gear.

An upduction is the opposite of a reduction, a larger gear driving a smaller gear. This is ineffcient and is typically something we want to avoid.

Deriving the relation between side by side gears:

_Assume: $T\_1$ is the smaller gear_
$$r=\text{radius}$$

$$v=r\omega$$
$$r=\frac{T}{2dp}$$
$$\text{since: }v_1=-v_2$$
$$r_1\omega_1=-r_2\omega_2$$
$$\frac{T_1}{2dp}\omega_1=-\frac{T_2}{2dp}\omega_2$$
$$T_1\omega_1=-T_2\omega_2$$
$$\omega_1=-\frac{T_2}{T_1}\omega_2$$

~~Deriving~~ the relation between stacked gears:

$$\omega_1=\omega_2$$

In a gearbox, the relations between gears typically alternate.

**EX:** Imagine a gearbox with four gears. The first gear is small and is directly driven by a motor. The second gear larger and is adjacent (side by side) to gear one. The third gear is another small gear but is stacked on top of gear two. The fourth gear is adjacent to gear three.

$$\omega_1=-\frac{T_2}{T_1}\omega_2$$
$$\omega_2=\omega_3$$
$$\omega_3=-\frac{T_4}{T_3}\omega_4$$
$$\omega_4=-\frac{T_3}{T_4}\omega_3$$
$$\omega_4=-\frac{T_3}{T_4}\omega_2$$
$$\omega_4=\frac{T_3}{T_4}\frac{T_1}{T_2}\omega_1$$

As you may have noticed, when gears alternate, the relationship between the angular speed of the first gear and the angular speed of the final gear are directly proportional. The coefficient on $\omega_1$ is equal to the product of the driving gears' tooth count divided by the product of driven gears' tooth count.

$$\omega_n=\omega_1\prod_{i=1}^{n/2}-\frac{T_{2i-1}}{T_{2i}}$$

Note: In a gearbox with an alternating structure, the number of stacked gears $n/2$ is given a special name as the "number of stages." Thus, the gearbox from the example above is a two stage gearbox.

#### Torque

The benefit of using gears to reduce the speed of a motor rather than using less electrical power is that a gearbox will increase the torque of the output in exchange for the lower speed.

**EX:** Deriving the relationship between gears and Torque:

Let: $\tau\_0=\text{motor torque}$

_Vector notation will be ommited to imporve readability_

_The following example continues off the previous one_

$$\vec{\tau}=\vec{r}\times \vec{F}$$
$$\tau_0=\tau_1$$
Since adjacent gears are touching at a point: $F_1=F_2=F_{12}$

![Gear Torque Diagram](/static/imgs/CAD/GearTorqueDiagram.png)

$$F_{12}=\tau_1/r_1$$
$$\tau_2=-F_{12}r_2$$
$$\tau_2=-\frac{r_2}{r_1}\tau_1$$
$$\tau_3=\tau_2$$
$$\tau_4=-\frac{r_4}{r_3}\tau_3$$
$$\tau_4=\frac{r_4}{r_3}\frac{r_2}{r_1}\tau_1$$

From this result, it is apparent that torque follows a similar relationship to that of angular velocity except that the coefficient on the input term in the torque equation is the reciprocal of the corresponding coefficient in the angular velocity equation.

Generalizing the formula found in the example:

$$\tau_n=\tau_1\prod_{i=1}^{n/2}-\frac{T_{2i}}{T_{2i-1}}$$

### Inertia

**Relevance:** When a gear spins, it will have a tendency to stay in its current state. This continued spinning, or lack thereof, affects the motion of a gear.

**Moment of Inertia**

The moment of inertia is defined as:

$$I=\int r^2 dm$$

The closer mass is located to the axis of rotation, the lower the moment of inertia, and the easier it is for the object to spin.

The following formula assumes that the gear has constant density and can be approximated as a cylinder.

$$I_{gear}=mr^2$$

Most Vex gears are half an inch wide and are made of 7075-T6 Aluminum. We can use this to find the mass of the gear in terms of the tooth count of the gear. Using the formula above in the "positioning" section, we can also find the radius of the gear in terms of tooth count.

| Equation                                  | Comment                         |
| ----------------------------------------- | ------------------------------- |
| $D=M/V$                                   | definition of density           |
| $M=D\*V$                                  | mass formula                    |
| $d=T/dp$                                  | gear diameter formula           |
| $r=T/2dp$                                 | gear radius fomula              |
| $V=\pi r^2 h$                             | volume of a cylinder            |
| $M=D\*\pi r^2 h$                          | mass of a cylinder              |
| $M=D\*\pi (T/2dp)^2 h$                    | mass of a gear                  |
| $M=D\pi h \frac{T^2}{4dp^2}$              | simplified                      |
| $D=2810\ kg/m^3$                          | density of 7075                 |
| $h=.5$                                    | thickness of a gear             |
| $M= 1103 \left( \frac{T^2}{dp^2} \right)$ | substitution and simplification |

The mass of a gear is 1103 time the square of the tooth count divided by the square of the diametrial pitch. This means that the moment of inertia ($kgm^2$) of a gear is:

$$I_{gear}=D\pi h \left( \frac{T^2}{4dp^2} \right)^2$$
$$I_{gear}=D\pi h \left( \frac{T^4}{16dp^4} \right)$$
$$I_{gear}=276 \left( \frac{T^4}{dp^4} \right)$$

**Inertia in Reductions**

From before we have:

$$\omega_1=-\frac{T_2}{T_1}\omega_2$$
$$\tau_2=-\frac{r_2}{r_1}\tau_1$$

Derivative with respect to time:
$$\alpha_1=-\frac{T_2}{T_1}\alpha_2$$

By definition:
$$\tau=I\alpha$$
$$r=T/2dp$$

Substituting:

$$\tau_1=I_1\alpha_1$$
$$\tau_2=I_2\alpha_2$$
$$I_2\alpha_2=-\frac{r_2}{r_1}I_1\alpha_1$$
$$I_2\alpha_2=-\frac{T_2/2dp}{T_1/2dp}I_1\alpha_1$$
$$I_2\alpha_2=-\frac{T_2}{T_1}I_1\alpha_1$$
$$I_2\alpha_2=-\frac{T_2}{T_1}I_1* \left( -\frac{T_2}{T_1}\alpha_2 \right)$$
$$I_2\alpha_2=\left( \frac{T_2}{T_1} \right)^2I_1 \alpha_2$$
$$I_2=\left( \frac{T_2}{T_1} \right)^2I_1$$
$$I_1=\left( \frac{T_1}{T_2} \right)^2I_2$$

Since the apparent moment of inertia is directly proportional to the square of the reduction, this derivation shows us that gear reducations allow inputs to drive systems with much larger moments as if they were small moments. In other words, reducations can help move a large flywheel with ease.

Although gears do add interia to a system, it is typically small in comparison to the benefit of the reduction.

Taking into account the inertia of the gears themselves:

$$I_{in}=\left( \frac{T_1}{T_2} \right)^2(I_{out}+I_{G2}) + I_{G1}$$
$$I_{out}=\left( I_{in}-I_{G1} \right) \left( \frac{T_2}{T_1} \right)^2-I_{G2}$$

## Credits

Initially written by [Michael Menezes](https://github.com/Menezmic21/) in September 2021
