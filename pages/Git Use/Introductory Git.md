# Introductory Git

## Git

Git is a version-control utility used for syncing code between multiple programmers _statefully_ (Requiring the state, or current condition of two or more synced items, to be similar). As a member of Texas Torque, you will be using Git to interact with the [Texas Torque codebase](https://github.com/TexasTorque) which is hosted on *GitHub*.

## Installation

**Linux:** Linux builds come with Git

**OSX:** OSX comes with Git

**Windows:** Download and install [Git for Windows](https://git-scm.com/download/win). After installing, you can open up the Git CLI by searching for it in the Windows menu.

## Cloning a Repository

In order to begin working on a project, you must first clone a repository. Think of cloning as downloading the files but with Git giving you extra capabilities. To initiate a clone, go to the directory you want the folder to be placed.

`cd path/to/directory`

Then initiate the clone using `git clone <url>`

`git clone --recursive https://github.com/TexasTorque/2020NewPeople`

The files will be downloaded into a folder sharing the same name as the repository.

## Using Branches

Unless otherwise told, you should be using `branches` while editing the code. Branches, like tree branches, separate the code from production and allows other teammates to check your code before `merging` (accepting) it.

To create a new branch, use the following command

`git branch <name>`

Then you can `checkout` the branch using

`git checkout <name>`

All changes you make will then be on the `<name>` branch

## Making Changes

Git tracks all changes that you create in your code automatically. When you are ready to propose a new change to the code for everyone else, you must first `stage` it. To do so, you will want to add the file(s) you changed.

To see the files you have edited, you can use the `git status` command

`git status`

To add a file to the stage you can use `git add <filename>`, such as

`git add Main.java`

## Committing Changes

Once you have added all the files you wish you can then `commit` the changes to be produced. Like a relationship, only commit once you are ready to push the files; if you mess up, use [this resource](https://dangitgit.com/en).

The command follows the following syntax

`git commit -m "Message describing what you did"`

## Pushing Changes

When you are ready to send your changes to the rest of the team you may `push` your changes. This moves the code you committed from your local computer to the cloud where others can see. Always confirm that your code builds properly `before` pushing your code. Please note you may commit multiple times before deciding to push your changes.

The command uses the following syntax

`git push origin <branch>`

If you are working directly on `master` (the default) then <branch> is master. Otherwise use the branch name that you checked out.

## Pulling Changes

If you have a repository already cloned, you can download the latest changes with `git pull`.

The syntax is

`git pull origin BRANCH`

If you are working directly on `master` (the default) then BRANCH is master. Otherwise use the branch name that you checked out.

*Note: Before starting a new branch, always checkout master and pull from it. Otherwise, you will run into issues.*

## Merging Code

Occasionally you will run into the problem of merging your code on your local machine. This occurs when you are trying to push your changes to a branch, but someone has already made changes that you don't have.

In order to solve this, `git pull` the branch in question.

This will open up a text editor you can use to create a merge message. Simply arrow key to the bottom, type in a quick message, and then hit the letter `q`.

Once complete, you will be allowed to push your code.

## Pulling Submodules

You can initially pull the code of submodules using the following

`git submodule update --init --recursive`

## Updating Submodules

You can update submodules using the following

`git submodule update --remote --merge`

## Credits

Initially written by [Jack Pittenger](https://github.com/realSaddy) in November 2020

Relocated by [Justus Languell](https://github.com/Juicestus) in September 2021
