# Docker环境搭建

作为服务器端Docker环境的搭建指引.

## 1. Download the image based on your demand. 

**Note:** Currently there are a lot of clean or setting finished images in the server.

```sudo docker pull ubuntu``` 

## 2. Run a docker container based on the image

```sudo docker run -itd --name=Name -p Port:22 --privileged --gpus=all  image_to_run```

**Note:** Replace these parameters, ```Name```: the name of the container, ```Port```: a valid and available port of the server, ```image_to_run```: the iamge you aim to run. (We get several ubuntu images, you could specify which one by ```ubuntu:tag```)

## 3. Attach the container

```sudo docker attach CONTAINER_JUST_CREATED```

## 4. (Optional, but Recommend) Set SSH for the container

This is the [instruction](http://www.yang99.top/index.php/archives/3/) for VScode and Remote SSH connection to the docker container. The port is what you choose in ```Step 2: $Port```
