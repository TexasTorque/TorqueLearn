# How to clone a Texas Torque repository

A TexasTorque Repository is structured very specificaly. Here is a high level overview of important directories and files in a repo.

```sh
├── build.gradle
├── gradle
├── gradlew
├── gradlew.bat
├── settings.gradle
├── src
│   └── main
│       ├── deploy
│       └── java
│           └── org
│               └── texastorque
│                   ├── Main.java
│                   ├── Robot.java
│                   ├── constants
│                   ├── inputs
│                   ├── subsystems
│                   ├── torquelib    <- Git Submodule
│                   └── util         <- Git Submodule 
└── vendordeps
    ├── Phoenix.json
    ├── REVRobotics.json
    ├── WPILibNewCommands.json
    └── WPILibOldCommands.json
```

Our repositories rely on two libraries, TorqueLib and Util. They are both Git Submodules, which are essentially repositories in a repository.

Before cloning a repository, you must be singed in to your GitHub account on the TexasTorque GitHub originazation. When the option becomes availavle. You also should have a directory (folder) of all your TexasTorque projects.

To clone the repo, you must use the command line pallet function called "**Git: Clone (recursive)"** and type in the URL of the GitHub repository you are cloning.

When the option becomes available, in the bottom right, select OPEN.

If you have done everything correctly, your source control tab should look like this:

![source control repo](/static/imgs/correctsourcecontrol.png)

## Credits

Initially written by [Justus Languell](https://github.com/Juicestus) in August 2021
