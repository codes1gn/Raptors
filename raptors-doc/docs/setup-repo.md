# 项目配置

## 1. Git 

*As a coder, definitely you have a [Github account](https://github.com/). If not, how you graduated???*

- ```apt install git-all``` to install **Git**

- ```git config --global user.name  用户名``` Name you used in registration.

- ``` git config --global user.email  邮箱``` Email you used in registration.


- ```ssh-keygen -t rsa -C "YOUR_EMAIL_ADDRESS_HERE"```  
**Note:** If you have already generated rsa file before, please check the output file name this time to avoid overwriting the previous rsa file.** 

- ```cat  ~/.ssh/id_rsa.pub```   
**Note:** This is the default rsa filename, change it if you use a different one.

Log in your [Github account](https://github.com/), open **Settings** -> **SSH and GPS keys** -> **New SSH key**, paste the content of ```id_rsa.pub ```, then click **Add SSH key**

---

## 2. MLIR

### 2.1 Install latest stable release CMake

- ```wget https://github.com/Kitware/CMake/releases/download/v3.23.3/cmake-3.23.3.tar.gz``` 

**Note:** Go to [CMake Org](https://cmake.org/download/) to check the link.

- ```tar -zxv -f cmake-3.23.3.tar.gz```

- ```cd cmake-3.23.3 && ./bootstrap```

**Note:** If get errors about C & C++ compilers, run command ```apt-get install gcc g++```

**Note:** If get the error about OpenSSL, run command ```apt install build-essential checkinstall zlib1g-dev -y``` and ```apt-get install libssl-dev```

***If <u>E:"Cannot open the lock files"</u>, you need to switch to ```root```, then re-run the command above.***

- ```make```

- ```make install``` 

Check the installation

- ```cmake --version```

### 2.2 MLIR installation

- ```apt install ninja-build```

This part follows [MLIR official installation document](https://mlir.llvm.org/getting_started/).

- ```git clone https://github.com/llvm/llvm-project.git```

- ```mkdir llvm-project/build```

- ```cd llvm-project/build```

```
cmake -G Ninja ../llvm \
    -DLLVM_ENABLE_PROJECTS=mlir \
    -DLLVM_BUILD_EXAMPLES=ON \
    -DLLVM_TARGETS_TO_BUILD="X86;NVPTX;AMDGPU" \
    -DCMAKE_BUILD_TYPE=Debug \
    -DLLVM_ENABLE_ASSERTIONS=ON
```

**Note:** Setting ```-DCMAKE_BUILD_TYPE=Debug``` will provide more error information. The default is ```-DCMAKE_BUILD_TYPE=Release```.

---

## 3. Raptors

### 3.1 Install Rust

- ```curl https://sh.rustup.rs -sSf | sh -s -- --help```

- ```git clone https://github.com/codes1gn/Raptors.git```

### 3.2 Clone Raptors repository

- ```cd Raptors```

- ```cargo install```

**Recommendation extension: rust-analyzer on VScode**

### 3.3 Rust学习路线

[Rust圣经](https://course.rs/about-book.html) + [Rust练习题](https://github.com/sunface/rust-by-practice) -> [The Rust Programming Language](https://doc.rust-lang.org/book/)
