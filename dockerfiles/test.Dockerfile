# Use an Ubuntu base image
FROM ubuntu:22.04

# Update package list and install OpenSSH Server
RUN apt-get update && apt-get install -y openssh-server

# Create the directory for SSH server and set permissions
RUN mkdir /var/run/sshd

# Set a password for the root user (Replace 'rootpassword' with a strong password)
RUN echo 'root:rootpassword' | chpasswd

# Enable root login via SSH
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

# Disable PAM authentication (optional)
RUN sed -i 's@session    required     pam_loginuid.so@session optional     pam_loginuid.so@g' /etc/pam.d/sshd

# Expose the SSH port
EXPOSE 22

# Start the SSH server
CMD ["/usr/sbin/sshd", "-D"]

