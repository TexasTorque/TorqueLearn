# How to use VSCode git

Pushing and pulling from GitHub is one of the many reasons that Torque is such a great, organized team. Through the use of uploading code to share and arhive, it become especially helpful to rookies like you and I (or even some senior programmers) to be able to reference older code to see how things should be setup, initiliazed, utilized, organized effectively, etc. It would be highly recommened for you to not only read through all of these documents to learn how to use GitHub efficiently, but to further research into more efficient or interesting ways to save time and energy.

Cloning Repositories:
The first thing that you need to do if you want to directly use GitHub pushing and pulling with VSCode is to copy the URL of the repository that you want to clone. Then, use the Ctrl+Shift+P (or Cmd+Shift+P) and then ensure that there is the > symbol in the popup text frame.
Next, type Git and you should see the option for Git Clone (Recursive). Click on this and then paste in the URL of your desired repository. 
*Note: If you do accidentally clone the repository without doing it recursively, then you will likely get errors from random classes/packages (submodules) not being imported. To fix this issue, you can open a terminal in VSCode and type "git update --init --recursive". This should solve your error.*

Pulling and Pushing Code:
If you have cloned a repo and are working on it with multiple people, then it is possible that someone else has been working on the same repository and pushed new changes to it. If this does happen, then it is important that you first pull the repository and merge it with your code to see if any errors or important discrepancies have occured. To do this, go to the Source Control tab on the left toolbar in VS Code and click on the three dots in the top right of the Source Control title. Click Stash, and then Stash. Next, you will likely be prompted to add a stash message, so type something summarizing what changes you made. Stashing your changes is important because if you just push or pull without stashing, you might realize that you need to undo the push/pull and if you have your changes stashed, then it is possible to go back to your code before the push/pull. Next, you will need to stage your changes. Click on the three dots in the top right of the title bar, click on Changes, and then Stage All Changes. This is essentially the first step of three to pushing code to git. After this, you can click on the message window beneath the Source Control title bar and type in a short, informative message (which can be the same as your stash message) and then you can do either Ctrl+Enter or Cmd+Enter for PC and Mac users respectively. After this, all you have to do is once again click on the three dots, and then click push!

## Credits

Initially written by [Omar Afzal](https://github.com/0mara) in August 2021
