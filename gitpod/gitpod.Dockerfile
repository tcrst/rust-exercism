
FROM gitpod/workspace-full

USER root

#RUN apt-get update && apt-get install -y gnupg software-properties-common curl
#RUN curl -fsSL https://apt.releases.hashicorp.com/gpg | apt-key add -
#RUN apt-add-repository "deb [arch=amd64] https://apt.releases.hashicorp.com $(lsb_release -cs) main"
#RUN apt-get update && apt-get install terraform

RUN curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose


RUN wget https://github.com/exercism/cli/releases/download/v3.0.13/exercism-3.0.13-linux-x86_64.tar.gz && \ 
    tar -xvf exercism-3.0.13-linux-x86_64.tar.gz && \
    mv exercism /usr/bin/exercism
    chmod +x /usr/bin/exercism