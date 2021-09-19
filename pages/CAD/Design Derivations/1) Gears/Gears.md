# Gears

## Intro

Gear are a simple yet effective way to trasfer rotaional power. Gears are used in all sorts of mechanisms from drivetrains to jointed arms and elevators.
s

## Concepts

[Positioning](#positioning)\
[Reductions](#reductions)\
[Inertia](#inertia)

### Positioning

**Relevance:** Gears that are positioned too close togehter can bind up and cause teeth to wear away faster.

Gears have an intrinsic property known as diamerial pitch *dp*. The diametrial pitch is defined as the the number of teeth on a gear *T* divided by the pitch diamter *d*. The pitch diameter is the diameter of a circle that intersects the teeth on the gear in the middle of the tooth height (technically it is when the tooth width is the same as the spacing between the teeth). Pitch diameter is easier explained with an image: 

![Thread Diagram](/layout/static/imgs/CAD/ThreadDiagram.gif)

Finding the diameter of a gear from dp:

$$dp=\text{teeth}/\text{diamter}$$
$$dp=T/d$$
$$d=T/dp$$

The recommended distance *rd* between gears is the center distance *cd* plus 0.008 inches.

$$cd=\frac{T_1+T_2}{2dp}$$
$$rd=\frac{T_1+T_2}{2dp}+0.008$$

### Reductions

**Relevance:** Gears can alter the speed, direction, and torque of an input. Understanding reductions provides insight on how well a system can perfom a task.

#### Rotational Velocity

Gear are typically arranged in two ways: side by side, or stacked.

![Gear Arrangement Diagram](/layout/static/imgs/CAD/GearArrangementDiagram.gif)

When gears are stacked, they have the same rotaional velocity $\omega$. When gears are side by side, they have the same tangental speed $|v|$.

A reduction occurs when a small gear "drives" a bigger gear aka. a small gear is next to a bigger gear.

An upduction is the opposite of a reduction, a larger gear driving a smaller gear. This is ineffcient and is typically something we want to avoid.

Deriving the relation between side by side gears:

**Assume: $T_1$ is the smaller gear*\
*$r=\text{radius}$

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

**Let: $\tau_0=\text{motor torque}$*\
**Vector notation will be ommited to imporve readability*\
**The following example continues off the previous one*

$$\vec{\tau}=\vec{r}\times \vec{F}$$
$$\tau_0=\tau_1$$
Since adjacent gears are touching at a point: $F_1=F_2=F_{12}$

![Gear Torque Diagram](/layout/static/imgs/CAD/GearTorqueDiagram.png)

$$F_{12}=\tau_1/r_1$$
$$\tau_2=F_{12}r_2$$
$$\tau_2=\frac{r_2}{r_1}\tau_1$$
$$\tau_3=\tau_2$$
$$\tau_4=\frac{r_4}{r_3}\tau_3$$
$$\tau_4=\frac{r_4}{r_3}\frac{r_2}{r_1}\tau_1$$

From this result, it is apparent that torque follows a similar relationship to that of angular velocity except that the coefficient on the input term in the torque equation is the reciprocal of the corresponding coefficient in the angular velocity equation. 

Generalizing the formula found in the example:

$$\tau_n=\tau_1\prod_{i=1}^{n/2}-\frac{T_{2i}}{T_{2i-1}}$$

### Inertia

**Relevance:** When a gear spins, it will have a tendency to keep spinning. This continued spinning after removal of power can skew percision.

TBD


## Credits

Initially written by [Michael Menezes](https://github.com/Menezmic21/) in September 2021
  