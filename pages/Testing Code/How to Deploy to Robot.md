# Deploy and run code on a robot

Deploying and running/controlling code on a robot is typically seen as a daunting task for beginners. However, as you'll soon read, it is not nearly as complicated as it might seem.

The first thing that you should do before you even turn on the robot to send your code to, is to open the WPILib Command Palette (Click on the white double with the red circle around it) and typing "Build Robot Code". What this will do is essentially a compile but with WPILib repos in mind. If your compile fails, check your code to see what is causing your error. This will ensure that you don't waste battery, time on the field, etc. If your code does compile/build succesfully, then you can move onto the next step.

The next step is to turn on the robot, go onto the computer with the changed code that had the successful build (It doesn't matter the OS, only that it has Wifi and preferrably isn't hosting a Live Share) and then connecting to the robots Wifi network. It should be 1477_RobotName. Once you get a connection, open up Visual Studio and again, click on the WPILib logo in the top right and type "Deploy Robot Code". Once you see a succesful message, you can now disconnect from the robot if you are on a Mac or Linux machine and then connect to the robots Wifi network on your driver station Windows machine. If you do not know what FRC Driver Station is, then you can read the documentation in the "Install FRC Gametools" file. 

The way that Torque robots run code is by having the code that it runs be kept on its RoboRio that lives on the robot. When you deploy new code, it overwrites the old code and keeps it in memory. This means that regardless of whether you turn the robot on or off, leave it in storage for 10 years, or use a different driver station computer with the robot than the one that you used to send the code, the robot will always have the code. The driver station computer merely sends controller signals to the robot and the robot itself will then interpret the signals. Ex: Driver Station: "A Button" Robot: "A Button" means "Shoot ball".

From here, all that you need to do is open FRC Driver Station, plug in an XBox controller, go to the USB devices tab on the left side and verify that when you perform inputs on the controller the computer sees them. If the computer is not seeing input, then change the USB port that FRC Driver Station is looking for the controller with. Next, you can go back to the first tab and ensure that all three of the boxes are green. If so, you can shout "Clear!" and either click the Enable button, or simultaneously press the open bracket, close bracket, and backslash key to enable the robot. To disable the robot (Which is something that you should always do before stepping onto the field to mess with or observe the robot), you can either press your Enter button or click the Disable button.

## Credits

Initially written by [Omar Afzal](https://github.com/0mara) in August 2021
