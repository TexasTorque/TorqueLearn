# How to use LiveShare

LiveShare is a real-time coding collaboration software developed by Microsoft to use in Visual Studio Code. LiveShare is very handy when multiple people need to work on a project simultaneously. Everyone on the LiveShare can read and edit code while seeing what others are working on.

## Install
To get started, install the LiveShare by Microsoft extension in the Visual Studio Extensions. After you install, the Live Share extension icon should pop up on your main panel. When you first use LiveShare, you might be asked to login with your Git credentials. This way, editors can collaborate on their Git accounts rather than using anonymous guest accounts.

## Start a Liveshare
To open a liveshare, open the project you'd like to work on and go to the LiveShare extension on your VSCode panel. Hit share, and a link will automatically be copied to your keyboard. Head to [Jack's Link Shortner](https://l.saddy.dev/) and paste the link. Enter the name you want your LiveShare to be. The password to the site is "lol". You will be generated a URL with the format https://l.saddy.dev/{name}. Anyone who would like to join the LiveShare session can enter that link, where they will be redirected to a VSCode desktop environment. The browser will then ask if they would like to open the project in VSCode. If this does not happen, they can copy the link and go to the LiveShare extension on VSCode and paste it in Join Session. You have now made a LiveShare session!

## Managing the LiveShare
When users enter the session, the host will have to accept them in with either "Read-Write" or "Read-Only" permissions. The host must always be connected to the internet, otherwise, the LiveShare will end. This is particularly a problem when the host is also the Driver Station for robot deployment. To circumvent this, try having a seperate device for the LiveShare host and the Driver Station. If this isn't a possibility, the host will have to restart the session after they connect to the Internet.


## Credits

Initially written by [Suhas Guddeti](https://github.com/Suhas44) in December 2021