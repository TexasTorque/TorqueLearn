# Introduction

Deviating from other articles in Torque Learn, we will show examples from langauges other than Java here.

Texas Torque typically uses Java. However C++ can be useful for applications where speed is absolutely essential. For example, computer vision applications. We prototype our opencv code in Python, and then convert it to C++ later. In order for a C++ program running on a coprocessor (ex. Raspberry Pi) to be able to communicate information back to the RoboRIO and main robot code, we use Network Tables. Network Tables are sort of like a shared JSON file hosted on the RoboRIO that can be written to programs running in other places in the robot's network. The following code examples below (working as of January 2022) show how to connect to network tables and write your first piece of data.

# C++
Note: you must include the Network Table header files.
Documentation: (https://first.wpi.edu/FRC/roborio/release/docs/cpp/classnt_1_1NetworkTableInstance.html#a02ea00d4a2a77b564d384b037f2045e6)

```c++
auto ntinst = nt::NetworkTableInstance::GetDefault(); // gets default instance of network tables
ntinst.StartClient("10.14.77.2"); // ip to connect to (HINT: YOU SHOULD be using a static ip 10.XX.XX.2 where XXXX is your team number)
//accessing table values
auto tb = ntinst.GetTable("TableName"); // should create the table on the network if it doesn't exist, returns a ptr to the table!
tb->PutNumber("entry1", 0.5);
```



