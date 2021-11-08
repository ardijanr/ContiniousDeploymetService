# Simple Continuous Deployment Web Application 

Written in actix web

Checks if the branch has a new commit and triggers docker-compose in the /home/deploy directory.

Returns an error if build failed.

How To:
 - Create the user deployer
 - Add ssh keys to deployer, root and github
 - Run the for compiling and placing the compiled binaries in their respective folders
 - Add ONLY the worker to sudoers with nopasswd such that it can run sudo docker-compose.
 - Run the scripts service scripts
 - Clone repos into /home/deployer/repos folder
 - Add github actions
 - Done

Remember its the name of the folder that determines the route.


Features needed:
 - Check for compose file existence, if not return as if the folder does not exist.
 - Check if containers are up report if not.
