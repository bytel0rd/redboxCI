# Dev Experience
- run RBX deploy -> the specific branch 

- RBX CI
    - Clone source code from repository
    - Run configuration commands
    - Deploy application [in various server nodes]

- RBX VPN/VPC
    - Create deployment VPC

- RBX LB
    - Mount application to the load balancer to RBX VPC


# Two binary client
- CI server and nodes
- CI clients


# Create A project
- The name of the project
- source control information
- build commands

project configuration are declared as a yaml file that's loaded in the application.


# Workspace
A temporary directory where your code is going to be build at.


# Clone from source control
- multiple source control [adapter pattern to clone using that the particular source control]
- clone into a workspace